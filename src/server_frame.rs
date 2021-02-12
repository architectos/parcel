use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub enum ChatMessageKind {
    Broadcast,
    Direct,
    Server,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ChatMessage {
    pub kind: ChatMessageKind,
    pub sender: String,
    pub text: String,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ServerFrame {
    pub id: u64,
    pub timestamp: u64,
    pub messages: Vec<ChatMessage>,
    pub position: [f32; 3],
    pub rotation: f32
}

impl ServerFrame {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Expected serialization of ServerMessage to be successful.")
    }
}
