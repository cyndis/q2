pub enum Message {
    Welcome(~[u8]),
    Ping(~[u8]),
    Join(~[u8], ~[u8]),
    Privmsg(~[u8], ~[u8], ~[u8]),
    Unknown(RawMessage)
}

/* I hate these clones but whatever for now */
pub fn parse_message(message: &[u8]) -> Option<Message> {
    parse_message_raw(message).and_then(|msg| {
        let RawMessage { prefix, command, parameters } = msg;
        if command.as_slice() == bytes!("001") {
            Some(Welcome(parameters[0]))
        }
        else if command.as_slice() == bytes!("PING") {
            Some(Ping(parameters[0]))
        }
        else if command.as_slice() == bytes!("JOIN") {
            let mut it = parameters.move_iter();
            match (prefix, it.next()) {
                (Some(prefix), Some(a)) => Some(Join(prefix, a)),
                _ => None
            }
        }
        else if command.as_slice() == bytes!("PRIVMSG") {
            let mut it = parameters.move_iter();
            match (prefix, it.next(), it.next()) {
                (Some(prefix), Some(a), Some(b)) => Some(Privmsg(prefix, a, b)),
                _ => None
            }
        }
        else { Some(Unknown(RawMessage { prefix: prefix, command: command, parameters: parameters })) }
    })
}

pub struct RawMessage {
    prefix: Option<~[u8]>,
    command: ~[u8],
    parameters: ~[~[u8]]
}

pub fn parse_message_raw(message: &[u8]) -> Option<RawMessage> {
    let mut prefix = None;
    let mut command = None;
    let mut parameters: ~[~[u8]] = ~[];
    let mut trailing = false;

    let mut iter = message.split(|&b| b == ' ' as u8).enumerate();
    loop {
        let (index, token) = match iter.next() {
            Some(x) => x,
            None    => break
        };

        if token.len() == 0 {
            if trailing {
                parameters.mut_last().unwrap().push(' ' as u8);
            }
            continue;
        }

        if token[0] == ':' as u8 {
            if index == 0 {
                // Source prefix
                prefix = Some(token.slice_from(1).to_owned());
                continue;
            } else if !trailing {
                // Start of trailing parameter
                trailing = true;
                parameters.push(token.slice_from(1).to_owned());
                continue;
            }
        }

        if index == 0 || (index == 1 && prefix.is_some()) {
            command = Some(token.to_owned());
            continue;
        }

        if trailing {
            parameters.mut_last().unwrap().push(' ' as u8);
            parameters.mut_last().unwrap().push_all(token);
        } else {
            parameters.push(token.to_owned());
        }
    }

    match command {
        Some(command) => Some(RawMessage { prefix: prefix,
                                           command: command,
                                           parameters: parameters }),
        None => None
    }
}

#[cfg(test)]
fn print_raw(m: &RawMessage) {
    use std::str;
    match m.prefix {
        Some(ref p) => println!(" PREFIX {}", str::from_utf8_lossy(*p)),
        None    => ()
    };
    println!("COMMAND {}", str::from_utf8_lossy(m.command));
    for param in m.parameters.iter() {
        println!("  PARAM {}", str::from_utf8_lossy(*param));
    }
}

#[test]
fn test_raw_welcome() {
    let m = parse_message_raw(bytes!("001 :Welcome to the Internet Relay Network a!b@c")).unwrap();
    print_raw(&m);

    assert!(m.prefix.is_none());
    assert_eq!(m.command.as_slice(), bytes!("001"));
    assert_eq!(m.parameters[0].as_slice(), bytes!("Welcome to the Internet Relay Network a!b@c"));
    assert_eq!(m.parameters.len(), 1);
}

#[test]
fn test_raw_join() {
    let m = parse_message_raw(bytes!(":WiZ JOIN #Twilight_zone")).unwrap();
    print_raw(&m);

    assert_eq!(m.prefix.as_ref().unwrap().as_slice(), bytes!("WiZ"));
    assert_eq!(m.command.as_slice(), bytes!("JOIN"));
    assert_eq!(m.parameters[0].as_slice(), bytes!("#Twilight_zone"));
    assert_eq!(m.parameters.len(), 1);
}

#[test]
fn test_ping() {
    let m = parse_message(bytes!("PING foobar.com")).unwrap();

    match m {
        Ping(ref x) if x.as_slice() == bytes!("foobar.com") => (),
        _ => fail!("no")
    }
}
