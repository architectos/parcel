use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum ChatTarget {
    Direct,
    Group
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Contents {
    Chat {
        target: ChatTarget,
        receiver: String,
        text: String
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub timestamp: u64,
    pub contents: Contents
}