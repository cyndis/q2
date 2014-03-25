use session::{Session, SessionCommand, SessionMessage};
use collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use sync::RWLock;
use std;

struct RemoteData {
    rx: Receiver<~str>,
    stream: TcpStream,
    session_id: Option<u64>
}

struct SessionData {
    session: Session,
    tx: Sender<SessionCommand>,
    rx: Receiver<SessionMessage>
}

pub struct RemoteControl {
    priv sessions: Option<HashMap<u64, SessionData>>,
    priv wakeup_rx: Option<Receiver<(Receiver<~str>, TcpStream)>>,
    priv wakeup_tx: Sender<(Receiver<~str>, TcpStream)>
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

    pub fn listen(&mut self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr);
        let mut acceptor = listener.listen();

        for stream in acceptor.incoming() {
            // TODO handle errors?
            let stream = stream.unwrap();
            let wakeup_tx = self.wakeup_tx.clone();
            spawn(proc() {
                let (remote_tx, remote_rx) = channel();
                wakeup_tx.send((remote_rx, stream.clone()));

                /* read loop */
            });
        }
    }

    pub fn run(&mut self) {
        let wakeup = self.wakeup_rx.take_unwrap();
        let sessions = self.sessions.take_unwrap();
        spawn(proc() {
            let mut remotes: ~[RemoteData] = ~[];
            loop {
                let sel = std::comm::Select::new();

                let mut session_handles = std::vec::with_capacity(sessions.len());
                for (session_id, data) in sessions.iter() {
                    let handle = sel.handle(&data.rx);
                    session_handles.push(handle);
                }

                let mut remote_handles = std::vec::with_capacity(remotes.len());
                for remote in remotes.iter() {
                    let handle = sel.handle(&remote.rx);
                    remote_handles.push(handle);
                }

                let mut wakeup_handle = sel.handle(&wakeup);

                unsafe { wakeup_handle.add(); }
                for handle in session_handles.mut_iter() { unsafe { handle.add(); } }
                for handle in remote_handles.mut_iter() { unsafe { handle.add(); } }

                let ready_id = sel.wait();
                if ready_id == wakeup_handle.id() {
                    // A remote was added, select again
                    continue;
                }

                for idx in session_handles.iter().position(|handle| handle.id() == ready_id).move_iter() {
                    // Handle received message from session with index `idx'
                    continue;
                }

                for idx in remote_handles.iter().position(|handle| handle.id() == ready_id).move_iter() {
                    // Handle received message from remote with index `idx'
                    continue;
                }
            }
        });
    }
}
