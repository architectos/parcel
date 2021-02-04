use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum ClientMessage {
    Chat {
        receiver: ChatReceiver,
        text: String
    }
}