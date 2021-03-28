use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Attributes {
    pub movement_speed: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Player {
    pub id: u64,
    pub transform: Transform,
    pub attributes: Attributes,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Flow {
    Normal,
    Overflow,
    Underflow,
}

impl Default for Flow {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ServerFrame {
    pub seq: u64,
    pub flow: Flow,
    pub player: Player,
    pub players: Vec<Player>,
}

impl ServerFrame {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Expected serialization of ServerMessage to be successful.")
    }
}
