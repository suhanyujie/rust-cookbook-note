use std::fmt::Debug;

#[derive(Debug)]
pub struct Helper {
    name: String,
}

impl Helper {
    pub fn new() -> Helper {
        Helper {
            name: String::from("bot1"),
        }
    }
}
