use std::io::net::{tcp, ip};
use std::io::{IoResult, IoError};
use std::{vec, io};
use std;

#[deriving(Clone)]
struct Connection {
    stream: tcp::TcpStream
}

static SP: &'static [u8] = bytes!(" ");
static CR: u8 = '\r' as u8;
static LF: u8 = '\n' as u8;

fn to_u8<T: ToStr>(x: &T) -> ~[u8] {
    x.to_str().bytes().collect()
}

impl Connection {
    pub fn connect(addr: ip::SocketAddr) -> IoResult<Connection> {
        tcp::TcpStream::connect(addr).map(|stream| Connection { stream: stream })
    }

    fn send(&mut self, strs: &[&[u8]]) -> IoResult<()> {
        for &s in strs.iter() {
            try!(self.stream.write(s))
        }
        self.stream.write(bytes!("\r\n"))
    }

    pub fn send_pass(&mut self, password: &[u8]) -> IoResult<()> {
        self.send([bytes!("PASS "), password])
    }

    pub fn send_nick(&mut self, nickname: &[u8]) -> IoResult<()> {
        self.send([bytes!("NICK "), nickname])
    }

    pub fn send_user(&mut self, username: &[u8], mode: u8, realname: &[u8]) -> IoResult<()> {
        self.send([bytes!("USER "), username, SP, to_u8(&mode).as_slice(), bytes!(" * :"), realname])
    }

    pub fn send_pong(&mut self, target: &[u8]) -> IoResult<()> {
        self.send([bytes!("PONG "), target])
    }

    pub fn send_join(&mut self, channel: &[u8]) -> IoResult<()> {
        self.send([bytes!("JOIN "), channel])
    }

    pub fn send_privmsg(&mut self, target: &[u8], message: &[u8]) -> IoResult<()> {
        self.send([bytes!("PRIVMSG "), target, bytes!(" :"), message])
    }

    pub fn send_quit(&mut self, message: &[u8]) -> IoResult<()> {
        self.send([bytes!("QUIT :"), message])
    }

    pub fn take_stream(self) -> tcp::TcpStream {
        self.stream
    }
}

pub enum ClientMessage {
    ConnectionError(IoError),
    Message(::parser::Message)
}

pub struct Client {
    priv conn_out: Option<Connection>,
    priv pipe: Option<Sender<ClientMessage>>
}

impl Client {
    pub fn new() -> (Client, Receiver<ClientMessage>) {
        let (tx, rx) = channel();
        (Client {
            conn_out: None,
            pipe: Some(tx)
         }, rx)
    }

    pub fn connect(&mut self, addr: ip::SocketAddr) {
        if self.conn_out.is_some() {
            println!("irc.client: tried to connect with connection already active");
            return;
        }

        match Connection::connect(addr) {
            Ok(c) => self.run(c),
            Err(e) => self.pipe.as_ref().unwrap().send(ConnectionError(e))
        }
    }

    pub fn disconnect(&mut self) {
        if self.conn_out.is_none() {
            println!("irc.client: tried to disconnect without connection active");
            return;
        }

        // Whoops, can't do anything! I may have mentioned Rust's crappy Socket API.
    }

    fn run(&mut self, conn: Connection) {
        self.conn_out = Some(conn.clone());
        let tx = self.pipe.take_unwrap();
        std::task::task().named("irc.client.ReaderTask").spawn(proc() {
            let mut conn_in = io::BufferedReader::new(conn.take_stream());
            let mut buf = vec::with_capacity(512);
            'done: loop {
                buf.clear();
                // this is kind of ugly
                loop {
                    match conn_in.read_byte() {
                        Ok(b) if b == LF => break,
                        Ok(b) if b == CR => (),
                        Ok(b)            => buf.push(b),
                        Err(e)           => { tx.send(ConnectionError(e)); break 'done; }
                    }
                }

                /* Message handling */

                match ::parser::parse_message(buf.as_slice()) {
                    Some(msg) => tx.send(Message(msg)),
                    _         => ()
                };
            }

        });
    }

    fn with_conn(&mut self, f: |&mut Connection| -> IoResult<()>) {
        if self.conn_out.is_none() {
            println!("irc.client: with_conn called with no connection");
            return;
        }
        if f(self.conn_out.as_mut().unwrap()).is_err() {
            // On error, close outbound socket. Error will hopefully be reported through
            // the inbound socket. Do this properly once the Rust socket API is better..
            self.conn_out.take_unwrap();
        }
    }

    pub fn register(&mut self, nickname: &[u8], username: &[u8], realname: &[u8]) {
        self.with_conn(|c| {
            c.send_nick(nickname).and_then(|_| c.send_user(username, 0, realname))
        })
    }

    pub fn privmsg(&mut self, target: &[u8], message: &[u8]) {
        self.with_conn(|c| {
            c.send_privmsg(target, message)
        })
    }

    pub fn join(&mut self, channel: &[u8]) {
        self.with_conn(|c| {
            c.send_join(channel)
        })
    }

    pub fn pong(&mut self, recipient: &[u8]) {
        self.with_conn(|c| { c.send_pong(recipient) } );
    }

    pub fn quit(&mut self, msg: &[u8]) {
        self.with_conn(|c| {
            c.send_quit(msg)
        });
    }
}

// ngircd
