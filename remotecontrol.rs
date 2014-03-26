use session::{Session, SessionCommand, SessionMessage};
use collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std;

/*
tag system:
    each remote gets an unique tag when it connects.
    parse_remote_packet inserts this tag into the tag field of each command-reply type
      session command it parses.
    pack_remote_packet will take the tag field from session replies that have it
      and return it along with the packed packet.
    the packet sending code will then check that the tag of the packet matches the
      tag of the remote before sending it.
*/

struct RemoteData {
    rx: Receiver<RemoteCommand>,
    stream: TcpStream,
    session_id: Option<u64>,
    tag: u64
}

impl RemoteData {
    fn write_packet(&mut self, packet: &[u8]) {
        self.stream.write_le_u32(packet.len() as u32);
        self.stream.write(packet);
    }

    fn write_error(&mut self, error: ~str) {
        self.write_packet(pack_remote_packet(RmError(error)).val0());
    }
}

pub struct SessionData {
    session: Option<Session>,
    tx: Sender<SessionCommand>,
    rx: Receiver<SessionMessage>
}

pub struct RemoteControl {
    priv sessions: Option<HashMap<u64, SessionData>>,
    priv wakeup_rx: Option<Receiver<(Receiver<RemoteCommand>, TcpStream, u64)>>,
    priv wakeup_tx: Sender<(Receiver<RemoteCommand>, TcpStream, u64)>
}

pub enum RemoteCommand {
    RcAttachSession(u64),
    RcSessionCommand(SessionCommand)
}

pub enum RemoteMessage {
    RmError(~str),
    RmSessionMessage(SessionMessage)
}

impl RemoteControl {
    pub fn new() -> RemoteControl {
        let (wtx, wrx) = channel();
        RemoteControl {
            sessions: Some(HashMap::new()),
            wakeup_rx: Some(wrx),
            wakeup_tx: wtx
        }
    }

    pub fn add_session(&mut self, id: u64, session: SessionData) {
        self.sessions.get_mut_ref().insert(id, session);
    }

    pub fn listen(&mut self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr);
        let mut acceptor = listener.listen();
        let mut next_tag = 0;

        for stream in acceptor.incoming() {
            // TODO handle errors?
            let stream = stream.unwrap();
            let wakeup_tx = self.wakeup_tx.clone();
            let tag = next_tag;
            next_tag += 1;
            std::task::task().named("RC-Reader").spawn(proc() {
                let mut stream = stream;
                let (remote_tx, remote_rx) = channel();
                wakeup_tx.send((remote_rx, stream.clone(), tag));

                loop {
                    let len = match stream.read_le_u32() {
                        Ok(len) => len,
                        Err(io_err) => fail!("Remote client failed during packet length: {}", io_err)
                    };
                    // TODO check for too large packets
                    // read_bytes -> read_exact in update
                    let packet = match stream.read_bytes(len as uint) {
                        Ok(packet) => packet,
                        Err(io_err) => fail!("Remote client failed during packet: {}", io_err)
                    };
                    let cmd = parse_remote_packet(packet, tag);
                    println!("<< {:?}", cmd);
                    match cmd {
                        Some(cmd) => remote_tx.send(cmd),
                        None => ()
                    }
                }
            });
        }
    }

    pub fn run(&mut self) {
        #[deriving(Show)]
        enum Source {
            FromSession(u64),
            FromRemote(uint),
            FromWakeup
        }
        let wakeup = self.wakeup_rx.take_unwrap();
        let mut sessions = self.sessions.take_unwrap();
        for (_, session) in sessions.mut_iter() {
            let sess = session.session.take_unwrap();
            spawn(proc() {
                let mut sess = sess;
                sess.run();
            });
        }
        spawn(proc() {
            let mut remotes: ~[RemoteData] = ~[];
            loop {
                let src = {
                    let sel = std::comm::Select::new();

                    let mut session_handles = std::vec::with_capacity(sessions.len());
                    for (session_id, data) in sessions.iter() {
                        let handle = sel.handle(&data.rx);
                        session_handles.push((*session_id, handle));
                    }

                    let mut remote_handles = std::vec::with_capacity(remotes.len());
                    {
                        for remote in remotes.iter() {
                            let handle = sel.handle(&remote.rx);
                            remote_handles.push(handle);
                        }
                    }

                    let mut wakeup_handle = sel.handle(&wakeup);

                    println!("Selecting wakeup + {} sessions + {} remotes", session_handles.len(), remote_handles.len());

                    unsafe { wakeup_handle.add(); }
                    for &(_, ref mut handle) in session_handles.mut_iter() { unsafe { handle.add(); } }
                    for handle in remote_handles.mut_iter() { unsafe { handle.add(); } }

                    let ready_id = sel.wait();
                    if ready_id == wakeup_handle.id() {
                        // A remote was added, select again
                        FromWakeup
                    } else {
                         session_handles.iter().find(|& &(_, ref handle)| handle.id() == ready_id)
                                               .map(|&(session_id, _)| FromSession(session_id))
                                               .unwrap_or_else(
                                || FromRemote(remote_handles.iter().position(|handle| handle.id() == ready_id).unwrap())
                                                )
                    }
                };

                println!("RC IN << {}", src);
                match src {
                    FromWakeup => {
                        // New remote added, spin again
                        // TODO What if this fails?
                        let (rx, stream, tag) = wakeup.recv();
                        remotes.push(RemoteData { rx: rx, stream: stream, session_id: None, tag: tag });
                    },
                    FromSession(session_id) => {
                        // Handle received message from session with id
                        // TODO what to do if this fails?
                        let msg = sessions.get(&session_id).rx.recv();
                        let (packet, tag) = pack_remote_packet(RmSessionMessage(msg));
                        for remote in remotes.mut_iter().filter(|r| r.session_id.is_some() && r.session_id.unwrap() == session_id) {
                            if tag.is_none() || tag.unwrap() == remote.tag {
                                remote.write_packet(packet);
                            }
                        }
                    },
                    FromRemote(remote_idx) => {
                        // Handle received message from remote with index `remote_idx'
                        let in_cmd = {
                            remotes[remote_idx].rx.recv_opt()
                        };
                        match in_cmd {
                            Some(cmd) => {
                                println!("RC ACK: {:?}", cmd);
                                match cmd {
                                    RcAttachSession(session_id) => {
                                        remotes[remote_idx].session_id = Some(session_id);
                                    },
                                    RcSessionCommand(sess_cmd) => {
                                        let sess = remotes[remote_idx].session_id.and_then(|sid|
                                            sessions.find(&sid));
                                        match sess {
                                            Some(sess) => sess.tx.send(sess_cmd),
                                            None => remotes[remote_idx].write_error(~"No session attached")
                                        }
                                    }
                                }
                            },
                            None      => {
                                println!("Remote is dead, purging");
                                remotes.remove(remote_idx);
                            }
                        }
                    }
                }
            }
        });
    }
}

/*
pub enum SessionCommand {
    NwConnect(u64, SocketAddr),
    NwRegister(u64, ~[u8]),
    NwJoinChannel(u64, ~[u8]),
    NwSendPrivmsg(u64, ~[u8], ~[u8])
}

pub enum SessionMessage {
    NetworkMessage(u64, ClientMessage)
}
*/

fn parse_remote_packet(packet: ~[u8], tag: u64) -> Option<RemoteCommand> {
    use session;

    // Temporary
    // Todo don't unwrap
    let tokens: ~[&str] = std::str::from_utf8(packet).unwrap().split('\n').collect();
    println!("PARSING TOKS {}", tokens);
    match tokens.as_slice() {
        ["attach", n] => {
            from_str(n).and_then(|n| Some(RcAttachSession(n)))
        },
        ["connect", network_id, addr] => {
            match (from_str(network_id), from_str(addr)) {
                (Some(network_id), Some(addr)) =>
                    Some(RcSessionCommand(session::NwConnect(network_id, addr))),
                _ => None
            }
        },
        ["register", network_id, nickname] => {
            match (from_str(network_id),) {
                (Some(network_id),) => Some(RcSessionCommand(session::NwRegister(network_id, nickname.to_owned().into_bytes()))),
                _ => None
            }
        }
        _ => None
    }
}

fn pack_remote_packet(msg: RemoteMessage) -> (~[u8], Option<u64>) {
    ((~"moi").into_bytes(), None)
}
