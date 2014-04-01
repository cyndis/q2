#[allow(deprecated_owned_vector)];

// for protobuf
#[feature(globs)];

extern crate irc;
extern crate collections;
extern crate sync;
extern crate serialize;
extern crate time;
extern crate protobuf;
extern crate sqlite3;

mod session;
mod network;
mod remotecontrol;
mod encoding;
mod buffer;
mod envelope;
mod database;

fn main() {
/*
    let (mut session, session_tx, session_rx) = session::Session::new();
    let network = network::Network::new();
    session.networks.insert(0, network);

    let mut remote = remotecontrol::RemoteControl::new();
    remote.add_session(0, remotecontrol::SessionData { session: Some(session), tx: session_tx, rx: session_rx });

    remote.run();
    remote.listen(from_str("0.0.0.0:9006").unwrap());*/

    let mut db = database::Database::open(~":memory:").ok().expect("Failed to open database");
    db.create_tables();
    db.bootstrap();

    let mut rc = db.load_core();

    rc.run();
    rc.listen(from_str("0.0.0.0:9006").unwrap());
}
