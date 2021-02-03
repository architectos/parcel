use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum ChatTarget {
    Direct,
    Group
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Contents {
    Chat {
        target: ChatTarget,
        receiver: String,
        text: String
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct ClientMessage {
    pub timestamp: u64,
    pub contents: Contents
}