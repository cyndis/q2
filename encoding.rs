pub struct Encoding;

impl Encoding {
    pub fn new() -> Encoding {
        Encoding
    }

    pub fn encode(&self, string: &~str) -> ~[u8] {
        string.bytes().collect()
    }
}
