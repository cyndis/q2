use irc;
use irc::client::{Client, ClientMessage};
use std::io::net::ip::SocketAddr;
//use buffer::Buffer;
use collections::HashMap;
use std;

pub struct Network {
    client: Client,
    rx: Receiver<ClientMessage>
//    buffers: ~[Buffer]
}

impl Network {
    pub fn new() -> Network {
        let (cli, rx) = Client::new();
        Network {
            client: cli,
            rx: rx
        }
    }

    pub fn handle_message(&mut self, tx: &Sender<SessionMessage>, id: u64) {
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

        tx.send(NetworkMessage(id, msg));
    }
}

pub struct Session {
    // TODO replace with hashmap
    networks: HashMap<u64, Network>,
    message_tx: Sender<SessionMessage>,
    command_rx: Receiver<SessionCommand>
}

pub enum SessionCommand {
    NwConnect(u64, SocketAddr),
    NwRegister(u64, ~[u8]),
    NwJoinChannel(u64, ~[u8]),
    NwSendPrivmsg(u64, ~[u8], ~[u8]),

    GetNetworkList(u64 /* tag */)
}

pub struct NetworkData {
    id: u64
}

pub enum SessionMessage {
    NetworkMessage(u64, ClientMessage),
    NetworkList(u64, ~[NetworkData])
}

impl Session {
    pub fn new() -> (Session, Sender<SessionCommand>, Receiver<SessionMessage>) {
        let (message_tx, message_rx) = channel();
        let (command_tx, command_rx) = channel();
        (Session {
            networks: HashMap::new(),
            message_tx: message_tx,
            command_rx: command_rx
         }, command_tx, message_rx)
    }

    pub fn run(&mut self) {
        enum MessageSource { FromNetwork(u64), FromRemote };
        loop {
            let source = {
                let sel = std::comm::Select::new();
                let mut handles = std::vec::with_capacity(self.networks.len());
                for (&id, network) in self.networks.iter() {
                    let handle = sel.handle(&network.rx);
                    handles.push((handle, id));
                }
                let mut cmd_handle = sel.handle(&self.command_rx);
                unsafe { cmd_handle.add(); }
                for &(ref mut handle, _) in handles.mut_iter() {
                    unsafe { handle.add(); }
                }
                let ready_id = sel.wait();
                if cmd_handle.id() == ready_id {
                    FromRemote
                } else {
                    FromNetwork(handles.iter().find(|& &(ref handle,_)| handle.id() == ready_id).map(|&(_, id)| id).unwrap())
                }
            };

            match source {
                FromRemote => self.handle_command(),
                FromNetwork(i) => self.networks.get_mut(&i).handle_message(&self.message_tx, i)
            }
        }
    }

    fn handle_command(&mut self) {
        let msg = match self.command_rx.recv_opt() {
            Some(m) => m,
            None    => return
        };

        // FIXME don't use get_mut! can fail!
        match msg {
            NwConnect(net, target) => self.networks.get_mut(&net).client.connect(target),
            NwRegister(net, nickname) => self.networks.get_mut(&net).client.register(nickname, nickname, nickname),
            NwJoinChannel(net, target) => self.networks.get_mut(&net).client.join(target),
            NwSendPrivmsg(net, target, message) => self.networks.get_mut(&net).client.privmsg(target, message),
            GetNetworkList(tag) => {
                let net_list = self.networks.iter().map(|(id, _net)| NetworkData { id: *id }).collect();
                self.message_tx.send(NetworkList(tag, net_list));
            }
        }
    }
}
