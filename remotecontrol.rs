use session::{Session, SessionCommand, SessionMessage};
use collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std;
use protobuf;
use protobuf::Message;
use network;
use session;
use buffer;

mod protocol;

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
        //self.write_packet(pack_remote_packet(RmError(error)).val0());
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
            std::task::task().named("remotecontrol.ReaderTask").spawn(proc() {
                let mut stream = stream;
                let (remote_tx, remote_rx) = channel();
                wakeup_tx.send((remote_rx, stream.clone(), tag));

                println!("New client");

                loop {
                    let len = match stream.read_le_u32() {
                        Ok(len) => len,
                        Err(io_err) => fail!("Remote client failed during packet length: {}", io_err)
                    };
                    println!("Remote waiting for packet of length {}", len);
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
            std::task::task().named("remotecontrol.SessionTask").spawn(proc() {
                let mut sess = sess;
                sess.run();
            });
        }
        std::task::task().named("remotecontrol.ControlTask").spawn(proc() {
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
                        let (rx, stream, tag) = match wakeup.recv_opt() {
                            Some(x) => x,
                            None    => { println!("!!! remotecontrol: wakeup is dead? !!!"); continue; }
                        };
                        remotes.push(RemoteData { rx: rx, stream: stream, session_id: None, tag: tag });
                    },
                    FromSession(session_id) => {
                        // Handle received message from session with id
                        // TODO what to do if this fails?
                        let msg = match sessions.get(&session_id).rx.recv_opt() {
                            Some(x) => x,
                            None    => { println!("!!! remotecontrol: session is dead? !!!"); continue; }
                        };
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

fn parse_remote_packet(packet: ~[u8], tag: u64) -> Option<RemoteCommand> {
    use session;

    let SC = RcSessionCommand;
    let NC = session::NetworkCommand;

    //println!("raw {}", packet);

    //let mut rd = std::io::MemReader::new(packet);
    //println!("eof {}", rd.eof());
    //let mut stream = protobuf::CodedInputStream::new(&mut rd);
    let mut cmd: protocol::RemoteCommand = protobuf::parse_from_bytes(packet);
    //cmd.merge_from(&mut stream);

    //println!("packet_type {:?}", cmd.packet_type);
    //println!("packet {:?}", cmd);

    if cmd.packet_type.is_none() { return None }
    let packet_type = cmd.packet_type.unwrap();

    match packet_type {
        protocol::AttachSession => {
            match cmd.attach_session {
                Some(protocol::AttachSessionT { session_id: Some(session_id) }) =>
                    Some(RcAttachSession(session_id)),
                _ => None
            }
        },
        protocol::GetNetworkList => {
            Some(RcSessionCommand(session::GetNetworkList(tag)))
        },
        protocol::Connect => {
            match (cmd.network_id, cmd.connect) {
                (Some(nid), Some(protocol::ConnectT { address: Some(address) })) =>
                    from_str(address).and_then(|x| Some(SC(NC(nid, network::Connect(x))))),
                _ => None
            }
        },
        protocol::Register => {
            match (cmd.network_id, cmd.register) {
                (Some(nid), Some(protocol::RegisterT { nickname: Some(nickname) })) =>
                    Some(SC(NC(nid, network::Register(nickname)))),
                _ => None
            }
        },
        protocol::JoinChannel => {
            match (cmd.network_id, cmd.join_channel) {
                (Some(nid), Some(protocol::JoinChannelT { channel: Some(channel) })) =>
                    Some(SC(NC(nid, network::JoinChannel(channel)))),
                _ => None
            }
        },
        protocol::SendPrivmsg => {
            match (cmd.network_id, cmd.send_privmsg) {
                (Some(nid), Some(protocol::SendPrivmsgT { target: Some(target), msg: Some(msg) })) =>
                    Some(SC(NC(nid, network::SendPrivmsg(target, msg)))),
                _ => None
            }
        },
        protocol::GetBufferList => {
            match cmd.network_id {
                Some(nid) => Some(SC(NC(nid, network::GetBufferList(tag)))),
                _ => None
            }
        }
    }
}

/*
pub enum RemoteCommand {
    RcAttachSession(u64),
    RcSessionCommand(SessionCommand)
}

pub enum RemoteMessage {
    RmError(~str),
    RmSessionMessage(SessionMessage)
}

pub enum SessionCommand {
    NetworkCommand(u64, network::Command),
    GetNetworkList(u64 tag)
}

pub enum SessionMessage {
    NetworkMessage(u64, network::Message),
    NetworkList(u64, ~[u64])
}

pub enum Command {
    Connect(SocketAddr),
    Register(~str),
    JoinChannel(~str),
    SendPrivmsg(~str, ~str),
    GetBufferList(u64 tag)
}

pub enum Message {
    Disconnected(~str),
    Connected,
    NewBuffer(u64, buffer::Role),
    BufferMessage(u64, buffer::Message),
    BufferList(u64 tag, ~[(u64, buffer::Role)])
}
*/

fn role_to_pbuf(role: buffer::Role) -> protocol::BufferRole {
    match role {
        buffer::Status =>
            protocol::BufferRole {
                buffer_type: Some(protocol::Status),
                name: Some(~"")
            },
        buffer::Channel(n) =>
            protocol::BufferRole {
                buffer_type: Some(protocol::Channel),
                name: Some(n)
            },
        buffer::Query(n) =>
            protocol::BufferRole {
                buffer_type: Some(protocol::Query),
                name: Some(n)
            }
    }
}
/*
fn netstate_to_pbuf(state: network::State) -> protocol::NetworkListT_NetworkState {
    match state {
        network::NetworkDisconnected => protocol::NetworkDisconnected,
        network::NetworkConnecting => protocol::NetworkConnecting,
        network::NetworkConnected => protocol::NetworkConnected
    }
}
*/
fn pack_remote_packet(msg: RemoteMessage) -> (~[u8], Option<u64>) {
    let mut pmsg: protocol::RemoteMessage = protobuf::Message::new();

    let mut out_tag = None;

    match msg {
        RmError(err) => {
            pmsg.set_packet_type(protocol::Error);
            pmsg.set_error(protocol::ErrorT { msg: Some(err) });
        },
        RmSessionMessage(msg) => {
            match msg {
                session::NetworkList(tag, data) => {
                    pmsg.set_packet_type(protocol::NetworkList);
                    out_tag = Some(tag);
                    for (id, state) in data.move_iter() {
                        pmsg.add_network_list(protocol::NetworkListT {
                            id: Some(id),
                            //state: Some(netstate_to_pbuf(state))
                        });
                    }
                },
                session::NetworkMessage(nid, msg) => {
                    pmsg.set_network_id(nid);
                    match msg {
                        network::Disconnected(reason) => {
                            pmsg.set_packet_type(protocol::Disconnected);
                            pmsg.set_disconnected(protocol::DisconnectedT { reason: Some(reason) });
                        },
                        network::Connected => {
                            pmsg.set_packet_type(protocol::Connected);
                        },
                        network::NewBuffer(bufid, role) => {
                            pmsg.set_packet_type(protocol::NewBuffer);
                            pmsg.set_new_buffer(protocol::NewBufferT {
                                id: Some(bufid),
                                role: Some(role_to_pbuf(role))
                            });
                        },
                        network::BufferList(tag, data) => {
                            pmsg.set_packet_type(protocol::BufferList);
                            out_tag = Some(tag);
                            for (bufid, role) in data.move_iter() {
                                pmsg.add_buffer_list(protocol::BufferListT {
                                    id: Some(bufid),
                                    role: Some(role_to_pbuf(role))
                                });
                            }
                        },
                        network::BufferMessage(bufid, msg) => {
                            pmsg.set_buffer_id(bufid);
                            pmsg.set_message_id(msg.id);
                            pmsg.set_message_time(msg.time_ns);
                            match msg.contents {
                                buffer::Information(info) => {
                                    pmsg.set_packet_type(protocol::Information);
                                    pmsg.set_information(protocol::InformationT { msg: Some(info) });
                                },
                                buffer::Join(who) => {
                                    pmsg.set_packet_type(protocol::Join);
                                    pmsg.set_join(protocol::JoinT { who: Some(who) });
                                },
                                buffer::Message(who, msg) => {
                                    pmsg.set_packet_type(protocol::Privmsg);
                                    pmsg.set_privmsg(protocol::PrivmsgT { who: Some(who), msg: Some(msg) });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if pmsg.is_initialized() {
        println!("OUT >> {:?}", pmsg);
        let out_str = pmsg.write_to_bytes();
        //println!("OUTBUF {}", out_str);
        (out_str, out_tag)
    } else {
        println!("FAIL: output empty");
        (~[], None)
    }
}
