pub enum Message {
    Welcome(~[u8]),
    Ping(~[u8]),
    Unknown(RawMessage)
}

pub fn parse_message(message: &[u8]) -> Option<Message> {
    parse_message_raw(message).and_then(|msg| {
        Some(
                 if msg.command.as_slice() == bytes!("001") { Welcome(msg.parameters[0]) }
            else if msg.command.as_slice() == bytes!("PING") { Ping(msg.parameters[0]) }
            else { Unknown(msg) }
        )
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
    let mut parameters = ~[];
    let mut trailing = false;

    let mut iter = message.split(|&b| b == ' ' as u8).enumerate();
    loop {
        let (index, token) = match iter.next() {
            Some(x) => x,
            None    => break
        };

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

    if command.is_some() {
        Some(RawMessage {
            prefix: prefix,
            command: command.unwrap(),
            parameters: parameters
        })
    } else {
        None
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
