use irc;
use irc::client::{Client, ClientMessage};
use std::io::net::ip::SocketAddr;
use std;
use encoding::{Encoding, IrcEncoding};
use buffer;
use envelope::Envelope;

pub struct EncodingPolicy {
    network: Encoding,
    outgoing: Encoding,
    incoming: Encoding
}

impl std::default::Default for EncodingPolicy {
    fn default() -> EncodingPolicy {
        EncodingPolicy {
            network: Encoding::new(),
            outgoing: Encoding::new(),
            incoming: Encoding::new()
        }
    }
}

pub enum State {
    NetworkDisconnected,
    NetworkConnecting,
    NetworkConnected
}

pub struct Configuration {
    server: SocketAddr,
    nickname: ~str
}

pub struct Network {
    client: Client,
    rx: Receiver<ClientMessage>,
    encoding: EncodingPolicy,
    buffers: ~[buffer::Buffer],
    state: State,
    nickname: Option<~[u8]>, // current nickname

    config: Option<Configuration>
}

impl Network {
    pub fn new() -> Network {
        let (cli, rx) = Client::new();
        Network {
            client: cli,
            rx: rx,
            encoding: std::default::Default::default(),
            buffers: ~[],
            state: NetworkDisconnected,
            nickname: None,
            config: None
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
                self.state = NetworkDisconnected;
                reply(Disconnected(err.desc.to_owned()))
            },
            irc::client::Message(msg) => {
                match msg {
                    irc::parser::Ping(ref sender) => self.client.pong(*sender),
                    irc::parser::Welcome(_) => {
                        self.state = NetworkConnected;
                        reply(Connected);
                        self.reply_buffer(reply, buffer::Status,
                                          buffer::Information(~"Welcome to IRC!"));
                    },
                    irc::parser::Join(who, channel) => {
                        let channel_l = self.encoding.network.decode(channel.irc_lowercase());
                        let who = self.encoding.network.decode(who);
                        self.reply_buffer(reply,
                                buffer::Channel(channel_l),
                                buffer::Join(who));
                    },
                    irc::parser::Privmsg(who, target, msg) => {
                        if target.irc_equal(self.nickname.get_ref()) {
                            let who_l = self.encoding.network.decode(who.irc_lowercase());
                            let who = self.encoding.network.decode(who);
                            let msg = self.encoding.incoming.decode(msg);
                            self.reply_buffer(reply, buffer::Query(who_l), buffer::Message(who, msg));
                        } else {
                            let target_l = self.encoding.network.decode(target.irc_lowercase());
                            let who = self.encoding.network.decode(who);
                            let msg = self.encoding.incoming.decode(msg);
                            self.reply_buffer(reply, buffer::Channel(target_l), buffer::Message(who, msg));
                        }
                    }
                    _ => ()
                }
            }
        }
    }

    pub fn handle_command(&mut self, cmd: Envelope<Command>, reply: |Envelope<Message>|) {
        let &Network { ref mut client, ref encoding, ref mut config, .. } = self;
        let &EncodingPolicy { network: ref en, outgoing: ref eo, .. } = encoding;

        let bare = cmd.bare();

        match cmd.contents {
            Connect => {
                match config {
                    &Some(ref config) => {
                        self.state = NetworkConnecting;
                        client.connect(config.server);
                        client.register(en.encode(&config.nickname),
                                        en.encode(&config.nickname),
                                        eo.encode(&config.nickname));
                    },
                    &None => ()
                }
            },
            JoinChannel(channel) => {
                client.join(en.encode(&channel));
            },
            SendPrivmsg(target, message) => {
                client.privmsg(en.encode(&target), eo.encode(&message));
            },
            GetBufferList(tag) => {
                let data = self.buffers.iter().map(|buf| (buf.id, buf.role.clone())).collect();
                reply(bare.copy_with(BufferList(tag, data)));
            },
            SetConfiguration(server, nickname) => {
                *config = Some(Configuration { server: server, nickname: nickname });
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
    Connect,
    JoinChannel(~str),
    SendPrivmsg(~str, ~str),
    GetBufferList(u64 /* tag */),
    SetConfiguration(SocketAddr, ~str)
}

pub enum Message {
    Disconnected(~str),
    Connected,
    NewBuffer(u64, buffer::Role),
    BufferMessage(u64, buffer::Message),
    BufferList(u64 /* tag */, ~[(u64, buffer::Role)])
}
