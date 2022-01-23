pub fn bobo(bobo: &str) -> String {
    format!("I am a {}", bobo)
}

pub struct Shop {
    pub name: String,
    no: u8
}

impl Shop {
    pub fn new(name: String, no: u8) -> Self {
        Shop {
            name: name,
            no: no
        }
    }
}