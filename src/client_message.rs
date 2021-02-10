use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema), schemars(deny_unknown_fields))]
pub enum ChatReceiver {
    All,
    Direct {
        target: String
    },
    Group {
        id: u32
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema), schemars(deny_unknown_fields))]
pub struct ChatMessage {
    pub receiver: ChatReceiver,
    pub text: String
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema), schemars(deny_unknown_fields))]
pub enum ClientMessage {
    Chat(ChatMessage)
}