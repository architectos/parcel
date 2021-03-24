use serde::{Deserialize, Serialize};

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
    pub seq: u64,
    pub movement: Movement,
    pub orientation: f32,
}
