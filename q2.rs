#[allow(deprecated_owned_vector)];

extern crate irc;

mod session;

fn main() {
    let (mut session, session_tx, session_rx) = session::Session::new();
    let mut network = session::Network::new();
    //network.client.connect(from_str("127.0.0.1:9048").unwrap());
    session.networks.push(network);

    spawn(proc() {
        let mut session = session;
        session.run();
    });

    let (con_tx, con_rx) = channel();
    spawn(proc() {
        loop {
            let line = std::io::stdin().read_line().unwrap();
            con_tx.send(line);
        }
    });

    loop {
        select!(
            msg = session_rx.recv() => {
                println!("session bus: {:?}", msg);
            },
            line = con_rx.recv() => {
                let words: ~[&str] = line.words().collect();

                match words[0] {
                    "connect" => session_tx.send(session::NwConnect(0, from_str(words[1]).unwrap())),
                    "register" => session_tx.send(session::NwRegister(0, words[1].to_owned().into_bytes())),
                    "join" => session_tx.send(session::NwJoinChannel(0, words[1].to_owned().into_bytes())),
                    "msg" => session_tx.send(session::NwSendPrivmsg(0, words[1].to_owned().into_bytes(), words.slice_from(2).connect(" ").into_bytes())),
                    _ => println!("unknown console input")
                }
            }
        )
    }
}
