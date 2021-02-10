use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema), schemars(deny_unknown_fields))]
pub enum ChatMessageKind {
    Broadcast,
    Direct,
    Server
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema), schemars(deny_unknown_fields))]
pub enum ServerMessage {
    Chat { kind: ChatMessageKind, sender: String, text: String },
    Empty {},
}

impl ServerMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Expected serialization of ServerMessage to be successful.")
    }
}
