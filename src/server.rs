use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Contents {
    Chat {
        sender: String,
        text: String
    },
    Empty
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct Message {
    pub timestamp: u64,
    pub contents: Contents
}