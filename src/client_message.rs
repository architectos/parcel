use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum ChatReceiverType {
    Direct,
    Group
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum ClientMessage {
    Chat {
        target: ChatReceiverType,
        receiver: String,
        text: String
    }
}