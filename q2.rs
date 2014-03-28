#[allow(deprecated_owned_vector)];

extern crate irc;
extern crate collections;
extern crate sync;
extern crate serialize;
extern crate time;

mod session;
mod network;
mod remotecontrol;
mod encoding;
mod buffer;

fn main() {
    let (mut session, session_tx, session_rx) = session::Session::new();
    let network = network::Network::new();
    session.networks.insert(0, network);

    let mut remote = remotecontrol::RemoteControl::new();
    remote.add_session(0, remotecontrol::SessionData { session: Some(session), tx: session_tx, rx: session_rx });

    remote.run();
    remote.listen(from_str("127.0.0.1:9006").unwrap());
}
