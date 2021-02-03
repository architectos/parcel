use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Contents {
    Chat {
        sender: String,
        text: String
    },
    Empty
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub timestamp: u64,
    pub contents: Contents
}