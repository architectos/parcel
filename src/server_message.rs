use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub enum ServerMessage {
    Chat {
        sender: String,
        text: String
    },
    Empty
}