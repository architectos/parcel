use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub enum ChatReceiver {
    Broadcast,
    Direct { target: String },
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ChatMessage {
    pub receiver: ChatReceiver,
    pub text: String,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Movement {
    pub north: bool,
    pub south: bool,
    pub west: bool,
    pub east: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ClientFrame {
    pub id: u64,
    pub timestamp: u64,
    pub messages: Vec<ChatMessage>,
    pub movement: Movement,
    pub rotation: f32
}
