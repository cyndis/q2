use irc;
use irc::client::{Client, ClientMessage};
use std;
use encoding::{Encoding, IrcEncoding};
use buffer;
use envelope::Envelope;
use database;

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

#[deriving(Clone)]
pub struct Configuration {
    server: ~str,
    nickname: ~str
}

pub struct Network {
    id: u64,
    db: database::Handle,
    client: Client,
    rx: Receiver<ClientMessage>,
    encoding: EncodingPolicy,
    buffers: ~[buffer::Buffer],
    state: State,
    nickname: Option<~[u8]>, // current nickname

    config: Option<Configuration>
}

impl Network {
    pub fn new(id: u64, db: database::Handle) -> Network {
        let (cli, rx) = Client::new();
        Network {
            id: id,
            db: db,
            client: cli,
            rx: rx,
            encoding: std::default::Default::default(),
            buffers: ~[],
            state: NetworkDisconnected,
            nickname: None,
            config: None
        }
    }

    pub fn handle_message(&mut self, reply: |msg::Message|) {
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
                reply(msg::Disconnected(err.desc.to_owned()))
            },
            irc::client::Message(msg) => {
                match msg {
                    irc::parser::Ping(ref sender) => self.client.pong(*sender),
                    irc::parser::Welcome(_) => {
                        self.state = NetworkConnected;
                        reply(msg::Connected);
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
                            self.reply_buffer(reply, buffer::Query(who_l), buffer::Privmsg(who, msg));
                        } else {
                            let target_l = self.encoding.network.decode(target.irc_lowercase());
                            let who = self.encoding.network.decode(who);
                            let msg = self.encoding.incoming.decode(msg);
                            self.reply_buffer(reply, buffer::Channel(target_l), buffer::Privmsg(who, msg));
                        }
                    }
                    _ => ()
                }
            }
        }
    }

    pub fn handle_command(&mut self, cmd: Envelope<msg::Command>, reply: |Envelope<msg::Message>|) {
        let &Network { ref mut client, ref encoding, ref mut config, .. } = self;
        let &EncodingPolicy { network: ref en, outgoing: ref eo, .. } = encoding;

        let bare = cmd.bare();

        match cmd.contents {
            msg::Connect => {
                match config {
                    &Some(ref config) => {
                        match from_str(config.server) {
                            Some(server) => {
                                self.state = NetworkConnecting;
                                client.connect(server);
                                client.register(en.encode(&config.nickname),
                                                en.encode(&config.nickname),
                                                eo.encode(&config.nickname));
                                reply(bare.copy_with(msg::Success));
                            },
                            None => reply(bare.copy_with(msg::Error(~"invalid server")))
                        }
                    },
                    &None => reply(bare.copy_with(msg::Error(~"network not configured")))
                }
            },
            msg::Disconnect => {
                self.state = NetworkDisconnected;
                client.quit(eo.encode(&~"So long, and thanks for all the fish."));
                client.disconnect();
                reply(bare.copy_with(msg::Success));
            }
            msg::JoinChannel(channel) => {
                client.join(en.encode(&channel));
                reply(bare.copy_with(msg::Success));
            },
            msg::SendPrivmsg(target, message) => {
                client.privmsg(en.encode(&target), eo.encode(&message));
                reply(bare.copy_with(msg::Success));
            },
            msg::GetBufferList => {
                let data = self.buffers.iter().map(|buf| (buf.id, buf.role.clone())).collect();
                reply(bare.copy_with(msg::BufferList(data)));
            },
            msg::SetConfiguration(cfg) => {
                self.db.update_network_configuration(self.id, &cfg);
                *config = Some(cfg);
                reply(bare.copy_with(msg::Success));
            },
            msg::GetConfiguration => {
                reply(bare.copy_with(msg::Configuration(config.clone())))
            },
            msg::GetBufferMessageRange(bufid, count, before_id) => {
                match self.buffers.mut_iter().find(|b| b.id == bufid) {
                    Some(buf) => reply(bare.copy_with(
                        msg::BufferMessageRange(bufid, buf.fetch_message_range(count, before_id)))),
                    None => reply(bare.copy_with(msg::Error(~"invalid buffer specified")))
                }
            }
        }
    }

    fn get_buffer<'a>(&'a mut self, reply: |msg::Message|, role: buffer::Role) -> &'a mut buffer::Buffer {
        let pos = {
            self.buffers.iter().position(|buffer| buffer.role == role)
        };
        match pos {
            Some(i) => &mut self.buffers[i],
            None    => {
                let buf = self.db.create_buffer(self.id, role);
                reply(msg::NewBuffer(buf.id, buf.role.clone()));
                self.buffers.push(buf);
                self.buffers.mut_last().unwrap()
            }
        }
    }

    fn reply_buffer(&mut self, reply: |msg::Message|, role: buffer::Role, cont: buffer::MessageContents) {
        let buffer = self.get_buffer(|x| reply(x), role);

        let buf_id = buffer.id;
        buffer.add(buffer::Message::now(cont),
            |msg| reply(msg::BufferMessage(buf_id, msg)));
    }
}

pub mod msg {
    use buffer;

    pub enum Command {
        Connect,
        Disconnect,
        JoinChannel(~str),
        SendPrivmsg(~str, ~str),
        GetBufferList,
        SetConfiguration(super::Configuration),
        GetConfiguration,
        GetBufferMessageRange(u64, uint, Option<u64>)
    }

    pub enum Message {
        Disconnected(~str),
        Connected,
        NewBuffer(u64, buffer::Role),
        BufferMessage(u64, buffer::Message),
        BufferList(~[(u64, buffer::Role)]),
        Error(~str),
        Success,
        Configuration(Option<super::Configuration>),
        BufferMessageRange(u64, ~[buffer::Message])
    }
}
