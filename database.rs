use sqlite3;
use sync;
use remotecontrol;
use session;
use network;
use buffer;

/*
    let db = sqlite3::open("asd.db").unwrap();

    db.exec("CREATE TABLE x (id INTEGER PRIMARY KEY, v INTEGER);");

    db.exec("INSERT INTO x VALUES (0, 42);");

    let cursor = db.prepare("SELECT v FROM x WHERE id = ?;", &None).unwrap();
    cursor.bind_param(1, &sqlite3::Integer(0));

    loop {
        let row = cursor.step_row().unwrap();

        match row {
            Some(r) => {
                let mut r = r;
                match r.pop(&~"v") {
                    Some(sqlite3::Integer(i)) => println!("{}", i),
                    _ => ()
                }
            },
            None => break
        }
    }
    */

pub struct Database {
    db: sqlite3::Database
}

impl Database {
    pub fn open(path: ~str) -> Result<Database,~str> {
        let db = match sqlite3::open(path) {
            Ok(db) => db,
            Err(x) => return Err(~"failed to open database")
        };

        Ok(Database { db: db })
    }

    pub fn create_tables(&mut self) {
        let q = include_str!("query_create_tables.sql");
        match self.db.exec(q) {
            Ok(_) => (),
            Err(e) => fail!("Creating tables failed: {}", self.db.get_errmsg())
        }
    }

    pub fn bootstrap(&mut self) {
        let q = include_str!("query_bootstrap.sql");
        self.db.exec(q).unwrap();
    }

    pub fn load_core(self) -> remotecontrol::RemoteControl {
        let mut handle = Handle { db: sync::RWArc::new(self) };

        let mut rc = remotecontrol::RemoteControl::new();

        handle.db.write(|db| {
            let mut ids = ~[];
            {
                let cursor = db.db.prepare("SELECT id FROM session;", &None).unwrap();
                // TODO error handling!!
                while cursor.step() == sqlite3::SQLITE_ROW {
                    let session_id = cursor.get_i64(0) as u64;
                    ids.push(session_id);
                }
            }

            for id in ids.move_iter() {
                let (mut session, session_tx, session_rx) = session::Session::new(handle.clone());

                load_session(&mut db.db, handle.clone(), id, &mut session);

                rc.add_session(id, remotecontrol::SessionData { session: Some(session),
                                                                tx: session_tx,
                                                                rx: session_rx });
            }
        });

        rc
    }
}

fn load_session(db: &mut sqlite3::Database, handle: Handle, id: u64, session: &mut session::Session) {
    let mut nets = ~[];

    {
        let cursor = db.prepare(
            "SELECT id, server, nickname FROM network WHERE session_id = ?;", &None
            ).unwrap();
        cursor.bind_param(1, &sqlite3::Integer64(id as i64));

        while cursor.step() == sqlite3::SQLITE_ROW {
            let network_id = cursor.get_i64(0) as u64;
            let server = match cursor.get_column_type(1) {
                sqlite3::SQLITE_NULL => None,
                _                    => Some(cursor.get_text(1))
            };
            let nickname = match cursor.get_column_type(2) {
                sqlite3::SQLITE_NULL => None,
                _                    => Some(cursor.get_text(2))
            };

            let mut network = network::Network::new(network_id, handle.clone());
            match (server, nickname) {
                (Some(server), Some(nickname)) =>
                    network.config = Some(network::Configuration { server: server,
                                                                    nickname: nickname }),
                    _ => ()
            }
            nets.push(network);
        }
    }

    for mut net in nets.move_iter() {
        load_buffers(db, handle.clone(), net.id, &mut net);

        session.networks.insert(net.id, net);
    }
}

fn load_buffers(db: &mut sqlite3::Database, handle: Handle, nid: u64, network: &mut network::Network) {
    let cursor = db.prepare(
        "SELECT id, role, name FROM buffer WHERE network_id = ?;", &None
        ).unwrap();
    cursor.bind_param(1, &sqlite3::Integer64(nid as i64));

    while cursor.step() == sqlite3::SQLITE_ROW {
        let buffer_id = cursor.get_i64(0) as u64;
        let role = cursor.get_int(1);
        let name = cursor.get_text(2);

        let role = match role {
            0 => buffer::Status,
            1 => buffer::Channel(name),
            2 => buffer::Query(name),
            _ => fail!("Invalid value in role enumeration in database")
        };

        let buffer = buffer::Buffer::create_repr(buffer_id, role, handle.clone());
        network.buffers.push(buffer);
    }
}

#[deriving(Clone)]
pub struct Handle {
    db: sync::RWArc<Database>
}

impl Handle {
    pub fn create_buffer(&mut self, nid: u64, role: buffer::Role) -> buffer::Buffer {
        self.db.write(|db| {
            let cursor = db.db.prepare(
                "INSERT INTO buffer (network_id, role, name) VALUES (?, ?, ?);", &None
                ).unwrap();
            cursor.bind_param(1, &sqlite3::Integer64(nid as i64));
            match role {
                buffer::Status => { cursor.bind_param(2, &sqlite3::Integer(0)); },
                buffer::Channel(ref n) => { cursor.bind_param(2, &sqlite3::Integer(1));
                                            cursor.bind_param(3, &sqlite3::Text(n.clone())); },
                buffer::Query(ref n) => { cursor.bind_param(2, &sqlite3::Integer(2));
                                          cursor.bind_param(3, &sqlite3::Text(n.clone())); },
            }
            // FIXME error checking
            cursor.step();
            // Clone of role is superfluous :( can be removed once we get update and get mutexes
            // ^ FIXME
            buffer::Buffer::create_repr(db.db.get_last_insert_rowid() as u64, role.clone(), self.clone())
        })
    }

    pub fn create_message(&mut self, bid: u64, mut message: buffer::Message) -> buffer::Message {
        self.db.write(|db| {
            let cursor = db.db.prepare(
                "INSERT INTO message (buffer_id, time_ns, type) VALUES (?, ?, ?);", &None
                ).unwrap();
            cursor.bind_param(1, &sqlite3::Integer64(bid as i64));
            cursor.bind_param(2, &sqlite3::Integer64(message.time_ns as i64));
            let msg_type = match message.contents {
                buffer::Information(..) => 0,
                buffer::Join(..) => 1,
                buffer::Privmsg(..) => 2
            };
            cursor.bind_param(3, &sqlite3::Integer64(msg_type));
            cursor.step();

            let msg_id = db.db.get_last_insert_rowid();

            match message.contents {
                buffer::Information(ref msg) => {
                    let cursor = db.db.prepare(
                        "INSERT INTO message_information (message_id, message) VALUES (?, ?)", &None
                        ).unwrap();
                    cursor.bind_param(1, &sqlite3::Integer64(msg_id));
                    cursor.bind_param(2, &sqlite3::Text(msg.clone()));
                    cursor.step();
                },
                buffer::Join(ref who) => {
                    let cursor = db.db.prepare(
                        "INSERT INTO message_join (message_id, who) VALUES (?, ?)", &None
                        ).unwrap();
                    cursor.bind_param(1, &sqlite3::Integer64(msg_id));
                    cursor.bind_param(2, &sqlite3::Text(who.clone()));
                    cursor.step();
                },
                buffer::Privmsg(ref who, ref msg) => {
                    let cursor = db.db.prepare(
                        "INSERT INTO message_privmsg (message_id, who, message) VALUES (?, ?, ?)", &None
                        ).unwrap();
                    cursor.bind_param(1, &sqlite3::Integer64(msg_id));
                    cursor.bind_param(2, &sqlite3::Text(who.clone()));
                    cursor.bind_param(3, &sqlite3::Text(msg.clone()));
                    cursor.step();
                }
            }

            message.id = msg_id as u64;
        });

        message
    }

    pub fn update_network_configuration(&mut self, nid: u64, config: &network::Configuration) {
        self.db.write(|db| {
            let cursor = db.db.prepare(
                "UPDATE network SET server = ?, nickname = ? WHERE id = ?;", &None
                ).unwrap();
            cursor.bind_param(1, &sqlite3::Integer64(nid as i64));
            cursor.bind_param(2, &sqlite3::Text(config.server.clone()));
            cursor.bind_param(3, &sqlite3::Text(config.nickname.clone()));
            cursor.step();
        });
    }

    pub fn fetch_messages_before(&mut self, buffer_id: u64, before_id: u64, count: uint)
        -> ~[buffer::Message]
    {
        self.db.write(|db| {
            let mut msgs = ~[];

            let cursor = db.db.prepare(include_str!("query_fetch_messages_before.sql"), &None).unwrap();
            // params: buffer_id, before_id, count
            cursor.bind_param(1, &sqlite3::Integer64(buffer_id as i64));
            cursor.bind_param(2, &sqlite3::Integer64(before_id as i64));
            cursor.bind_param(3, &sqlite3::Integer(count as int));

            // cols:   id, time_ns, type,
  //information.message,
  //'join'.who,
  //privmsg.who, privmsg.message
            while cursor.step() == sqlite3::SQLITE_ROW {
                let msg = buffer::Message {
                    id: cursor.get_i64(0) as u64,
                    time_ns: cursor.get_i64(1) as u64,
                    contents: match cursor.get_int(2) {
                        0 => buffer::Information(cursor.get_text(3)),
                        1 => buffer::Join(cursor.get_text(4)),
                        2 => buffer::Privmsg(cursor.get_text(5), cursor.get_text(6)),
                        _ => fail!("Invalid message enumeration value in database")
                    }
                };
                msgs.push(msg);
            }

            msgs
        })
    }

    // FIXME remove duplication
    pub fn fetch_latest_messages(&mut self, buffer_id: u64, count: uint) -> ~[buffer::Message] {
        self.db.write(|db| {
            let mut msgs = ~[];

            let cursor = db.db.prepare(include_str!("query_fetch_latest_messages.sql"), &None).unwrap();
            // params: buffer_id, before_id, count
            cursor.bind_param(1, &sqlite3::Integer64(buffer_id as i64));
            cursor.bind_param(2, &sqlite3::Integer(count as int));

            // cols:   id, time_ns, type,
  //information.message,
  //'join'.who,
  //privmsg.who, privmsg.message
            while cursor.step() == sqlite3::SQLITE_ROW {
                let msg = buffer::Message {
                    id: cursor.get_i64(0) as u64,
                    time_ns: cursor.get_i64(1) as u64,
                    contents: match cursor.get_int(2) {
                        0 => buffer::Information(cursor.get_text(3)),
                        1 => buffer::Join(cursor.get_text(4)),
                        2 => buffer::Privmsg(cursor.get_text(5), cursor.get_text(6)),
                        _ => fail!("Invalid message enumeration value in database")
                    }
                };
                msgs.push(msg);
            }

            msgs
        })
    }
}
