pub struct Envelope<T> {
    client_tag: Option<u64>,
    remote_tag: Option<u64>,
    contents: T
}

impl<T> Envelope<T> {
    pub fn empty(x: T) -> Envelope<T> {
        Envelope { client_tag: None, remote_tag: None, contents: x }
    }

    pub fn encapsulate<U>(self, f: fn(T) -> U) -> Envelope<U> {
        let Envelope { client_tag, remote_tag, contents } = self;
        Envelope { client_tag: client_tag, remote_tag: remote_tag, contents: f(contents) }
    }

    pub fn copy_with<U>(&self, x: U) -> Envelope<U> {
        Envelope { client_tag: self.client_tag.clone(), remote_tag: self.remote_tag.clone(),
                   contents: x }
    }

    pub fn bare(&self) -> Envelope<()> {
        Envelope { client_tag: self.client_tag.clone(), remote_tag: self.remote_tag.clone(),
                   contents: () }
    }
}
