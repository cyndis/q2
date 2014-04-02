use std;

pub struct Encoding;

impl Encoding {
    pub fn new() -> Encoding {
        Encoding
    }

    pub fn encode(&self, string: &~str) -> ~[u8] {
        string.bytes().collect()
    }

    pub fn decode(&self, string: &[u8]) -> ~str {
        std::str::from_utf8_lossy(string).into_owned()
    }
}

pub trait IrcEncoding {
    fn irc_lowercase(&self) -> Self;
    fn irc_equal(&self, other: &Self) -> bool;
}

impl IrcEncoding for ~[u8] {
    fn irc_lowercase(&self) -> ~[u8] {
        self.clone()
        /*
        self.iter().map(|&b| {
            match b {
                0x5B..0x5E | 0x61..0x7A => b + 32,
                _                       => b
            }
        }).collect()
        */
    }

    fn irc_equal(&self, other: &~[u8]) -> bool {
        self.irc_lowercase() == other.irc_lowercase()
    }
}
