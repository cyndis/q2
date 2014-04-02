use time;
use database;

#[deriving(Eq, Clone)]
pub enum Role {
    Status,
    Channel(~str),
    Query(~str)
}

#[deriving(Clone)]
pub struct Message {
    id: u64,
    time_ns: u64,
    contents: MessageContents
}

fn time_now() -> u64 {
    let t = time::get_time();
    t.sec as u64 * 1000000000 + t.nsec as u64
}

impl Message {
    pub fn now(cont: MessageContents) -> Message {
        Message {
            id: 0,
            time_ns: time_now(),
            contents: cont
        }
    }
}

#[deriving(Clone)]
pub enum MessageContents {
    Information(~str),
    Join(~str), // who
    Privmsg(~str, ~str) // sender, message
}

/*
mod memory {
    pub struct Buffer {
        id: u64,
        role: Role,
        messages: ~[Message],
        last_read: Option<u64>,
        next_id: u64
    }

    impl Buffer {
        pub fn empty(id: u64, role: Role) -> Buffer {
            Buffer {
                id: id,
                messages: ~[],
                last_read: None,
                next_id: 0,
                role: role
            }
        }

        pub fn with_messages(id: u64, role: Role, messages: ~[Message]) -> Buffer {
            Buffer {
                id: id,
                messages: messages,
                last_read: None,
                next_id: 0,
                role: role
            }
        }

        pub fn add(&mut self, mut msg: Message, cb: |&Message|) -> u64 {
            msg.id = self.next_id;
            self.next_id += 1;
            self.messages.push(msg);
            cb(self.messages.last().unwrap());
            self.next_id - 1
        }

        pub fn set_last_read(&mut self, last_read: u64) {
            self.last_read = Some(last_read);
        }
    }
}
*/

pub struct Buffer {
    id: u64,
    role: Role,
    stored_messages: u64,
    db: database::Handle
}

impl Buffer {
    pub fn create_repr(id: u64, role: Role, stored_messages: u64, db: database::Handle) -> Buffer {
        Buffer {
            id: id,
            role: role,
            stored_messages: stored_messages,
            db: db
        }
    }

    pub fn add(&mut self, msg: Message, cb: |Message|) {
        let msg = self.db.create_message(self.id, msg);
        cb(msg);
        self.stored_messages += 1;
    }

    pub fn fetch_message_range(&mut self, count: uint, before_id: Option<u64>) -> ~[Message] {
        match before_id {
            Some(id) => self.db.fetch_messages_before(self.id, id, count),
            None     => self.db.fetch_latest_messages(self.id, count)
        }
    }
}
