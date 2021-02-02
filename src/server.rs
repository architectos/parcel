use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Contents {
    Chat {
        sender: String,
        text: String
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    timestamp: u64,
    contents: Contents
}