use collections::HashMap;
use std;
use network;
use envelope::Envelope;

pub struct Session {
    networks: HashMap<u64, network::Network>,
    message_tx: Sender<Envelope<msg::Message>>,
    command_rx: Receiver<Envelope<msg::Command>>
}

pub mod msg {
    use network;

    pub enum Command {
        NetworkCommand(u64, network::msg::Command),
        GetNetworkList
    }

    pub enum Message {
        NetworkMessage(u64, network::msg::Message),
        NetworkList(~[(u64, network::State)]),
        Error(~str),
        Success
    }
}

impl Session {
    pub fn new() -> (Session, Sender<Envelope<msg::Command>>, Receiver<Envelope<msg::Message>>) {
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
                FromNetwork(i) => {
                    let &Session { ref mut networks, ref message_tx, .. } = self;
                    networks.get_mut(&i).handle_message(|msg| message_tx.send(Envelope::empty(msg::NetworkMessage(i, msg))))
                }
            }
        }
    }

    fn handle_command(&mut self) {
        let msg = match self.command_rx.recv_opt() {
            Some(m) => m,
            None    => return
        };

        let bare = msg.bare();

        match msg.contents {
            msg::NetworkCommand(id, cmd) => {
                let &Session { ref mut networks, ref message_tx, .. } = self;
                match networks.find_mut(&id) {
                    Some(nw) => nw.handle_command(bare.copy_with(cmd), |msg| message_tx.send(msg.encapsulate(|m| msg::NetworkMessage(id, m)))),
                    None     => println!("Remote used invalid network id")
                }
            },
            msg::GetNetworkList => {
                let net_list = self.networks.iter().map(|(id, net)| (*id, net.state)).collect();
                self.message_tx.send(msg.copy_with(msg::NetworkList(net_list)));
            }
        }
    }
}
