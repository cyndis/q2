use irc;
use irc::client::{Client, ClientMessage};
use std::io::net::ip::SocketAddr;
//use buffer::Buffer;
use std;

pub struct Network {
    id: u64,
    client: Client,
    rx: Receiver<ClientMessage>
//    buffers: ~[Buffer]
}

impl Network {
    pub fn new() -> Network {
        let (cli, rx) = Client::new();
        Network {
            id: 0, // FIXME
            client: cli,
            rx: rx
        }
    }

    pub fn handle_message(&mut self, tx: &Sender<SessionMessage>) {
        let msg = match self.rx.recv_opt() {
            Some(m) => m,
            None    => return
        };
        println!("{:?}", msg);

        match msg {
            irc::client::ConnectionError(_) => {
                // Client has died, make a new one
                println!("Recreating backing client");
                let (cli, rx) = Client::new();
                self.client = cli;
                self.rx = rx;
            },
            irc::client::Message(ref msg) => {
                match *msg {
                    irc::parser::Welcome(_) => println!("Welcome to IRC!"),
                    irc::parser::Ping(ref sender) => self.client.pong(*sender),
                    _ => ()
                }
            }
        }

        tx.send(NetworkMessage(self.id, msg));
    }
}

pub struct Session {
    // TODO replace with hashmap
    networks: ~[Network],
    message_tx: Sender<SessionMessage>,
    command_rx: Receiver<SessionCommand>
}

pub enum SessionCommand {
    NwConnect(u64, SocketAddr),
    NwRegister(u64, ~[u8]),
    NwJoinChannel(u64, ~[u8]),
    NwSendPrivmsg(u64, ~[u8], ~[u8])
}

pub enum SessionMessage {
    NetworkMessage(u64, ClientMessage)
}

impl Session {
    pub fn new() -> (Session, Sender<SessionCommand>, Receiver<SessionMessage>) {
        let (message_tx, message_rx) = channel();
        let (command_tx, command_rx) = channel();
        (Session {
            networks: ~[],
            message_tx: message_tx,
            command_rx: command_rx
         }, command_tx, message_rx)
    }

    pub fn run(&mut self) {
        enum MessageSource { FromNetwork(uint), FromRemote };
        loop {
            let source = {
                let sel = std::comm::Select::new();
                let mut handles = std::vec::with_capacity(self.networks.len());
                for network in self.networks.iter() {
                    let handle = sel.handle(&network.rx);
                    handles.push(handle);
                }
                let mut cmd_handle = sel.handle(&self.command_rx);
                unsafe { cmd_handle.add(); }
                for handle in handles.mut_iter() {
                    unsafe { handle.add(); }
                }
                let ready_id = sel.wait();
                if cmd_handle.id() == ready_id {
                    FromRemote
                } else {
                    FromNetwork(handles.iter().position(|handle| handle.id() == ready_id).unwrap())
                }
            };

            match source {
                FromRemote => self.handle_command(),
                FromNetwork(i) => self.networks[i].handle_message(&self.message_tx)
            }
        }
    }

    fn handle_command(&mut self) {
        let msg = match self.command_rx.recv_opt() {
            Some(m) => m,
            None    => return
        };

        match msg {
            NwConnect(net, target) => self.networks[net].client.connect(target),
            NwRegister(net, nickname) => self.networks[net].client.register(nickname, nickname, nickname),
            NwJoinChannel(net, target) => self.networks[net].client.join(target),
            NwSendPrivmsg(net, target, message) => self.networks[net].client.privmsg(target, message)
        }
    }
}
