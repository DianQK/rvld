pub struct File {
    pub name: String,
    pub contents: Vec<u8>,
}

impl File {
    pub fn new(name: String, contents: Vec<u8>) -> Self {
        Self { name, contents }
    }
}
