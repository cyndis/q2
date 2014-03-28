use irc;
use irc::client::{Client, ClientMessage};
use std::io::net::ip::SocketAddr;
use std;
use encoding::Encoding;
use buffer;

pub struct EncodingPolicy {
    network: Encoding,
    outgoing: Encoding
}

impl std::default::Default for EncodingPolicy {
    fn default() -> EncodingPolicy {
        EncodingPolicy {
            network: Encoding::new(),
            outgoing: Encoding::new()
        }
    }
}

pub struct Network {
    client: Client,
    rx: Receiver<ClientMessage>,
    encoding: EncodingPolicy,
    buffers: ~[buffer::Buffer]
}

impl Network {
    pub fn new() -> Network {
        let (cli, rx) = Client::new();
        Network {
            client: cli,
            rx: rx,
            encoding: std::default::Default::default(),
            buffers: ~[]
        }
    }

    pub fn handle_message(&mut self, reply: |Message|) {
        let msg = match self.rx.recv_opt() {
            Some(m) => m,
            None    => return
        };
        println!("{:?}", msg);

        match msg {
            irc::client::ConnectionError(err) => {
                // Client has died, make a new one
                println!("Recreating backing client");
                let (cli, rx) = Client::new();
                self.client = cli;
                self.rx = rx;
                reply(Disconnected(err.desc.to_owned()))
            },
            irc::client::Message(ref msg) => {
                match *msg {
                    irc::parser::Ping(ref sender) => self.client.pong(*sender),
                    irc::parser::Welcome(_) => {
                        reply(Connected);
                        self.reply_buffer(reply, buffer::Status,
                                          buffer::Information(~"Welcome to IRC!"));
                    },
                    _ => ()
                }
            }
        }
    }

    pub fn handle_command(&mut self, cmd: Command, reply: |Message|) {
        let &Network { ref mut client, ref encoding, .. } = self;
        let &EncodingPolicy { network: ref en, outgoing: ref eo, .. } = encoding;

        match cmd {
            Connect(addr) => {
                client.connect(addr);
            },
            Register(nickname) => {
                client.register(
                    en.encode(&nickname),
                    en.encode(&nickname),
                    eo.encode(&nickname)
                );
            },
            JoinChannel(channel) => {
                client.join(en.encode(&channel));
            },
            SendPrivmsg(target, message) => {
                client.privmsg(en.encode(&target), eo.encode(&message));
            },
            GetBufferList(tag) => {
                let data = self.buffers.iter().map(|buf| (buf.id, buf.role.clone())).collect();
                reply(BufferList(tag, data));
            }
        }
    }

    fn get_buffer<'a>(&'a mut self, reply: |Message|, role: buffer::Role) -> &'a mut buffer::Buffer {
        let pos = {
            self.buffers.iter().position(|buffer| buffer.role == role)
        };
        match pos {
            Some(i) => &mut self.buffers[i],
            None    => {
                let buf = buffer::Buffer::empty(0 /* FIXME */, role);
                reply(NewBuffer(buf.id, buf.role.clone()));
                self.buffers.push(buf);
                self.buffers.mut_last().unwrap()
            }
        }
    }

    fn reply_buffer(&mut self, reply: |Message|, role: buffer::Role, cont: buffer::MessageContents) {
        let buffer = self.get_buffer(|x| reply(x), role);

        let buf_id = buffer.id;
        buffer.add(buffer::Message::now(cont),
            |msg| reply(BufferMessage(buf_id, (*msg).clone())));
    }
}

pub enum Command {
    Connect(SocketAddr),
    Register(~str),
    JoinChannel(~str),
    SendPrivmsg(~str, ~str),
    GetBufferList(u64 /* tag */)
}

pub enum Message {
    Disconnected(~str),
    Connected,
    NewBuffer(u64, buffer::Role),
    BufferMessage(u64, buffer::Message),
    BufferList(u64 /* tag */, ~[(u64, buffer::Role)])
}
