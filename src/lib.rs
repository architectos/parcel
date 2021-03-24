pub use bincode;
pub use serde;

#[cfg(feature = "json-schema")]
pub use schemars;

pub mod client_frame;
pub use client_frame::ClientFrame;

pub mod server_frame;
pub use server_frame::ServerFrame;

#[cfg(test)]
mod tests {
    use crate::server_frame::Transform;

    use super::*;
    use bincode::{deserialize, serialize};

    #[test]
    fn test_client_message_serialize() -> () {
        serialize(&ClientFrame {
            seq: 0,
            displacement: [0.0, 0.0],
        })
        .unwrap();

        serialize(&ClientFrame {
            seq: 0,
            displacement: [0.0, 0.0],
        })
        .unwrap();
    }

    #[test]
    fn test_client_message_deserialize() -> () {
        deserialize::<ClientFrame>(
            &serialize(&ClientFrame {
                seq: 0,
                displacement: [0.0, 0.0],
            })
            .unwrap(),
        )
        .unwrap();

        deserialize::<ClientFrame>(
            &serialize(&ClientFrame {
                seq: 0,
                displacement: [0.0, 0.0],
            })
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_server_message_serialize() -> () {
        serialize(&ServerFrame {
            seq: 0,
            flow: server_frame::Flow::Normal,
            attributes: server_frame::Attributes {
                movement_speed: 0.0,
            },
            transform: Transform {
                position: [0.0, 0.0, 0.0],
                rotation: 0.0,
                scale: 0.0,
            },
            players: Vec::new(),
        })
        .unwrap();
    }

    #[test]
    fn test_server_message_deserialize() -> () {
        deserialize::<ServerFrame>(
            &serialize(&ServerFrame {
                seq: 0,
                flow: server_frame::Flow::Normal,
                attributes: server_frame::Attributes {
                    movement_speed: 0.0,
                },
                transform: Transform {
                    position: [0.0, 0.0, 0.0],
                    rotation: 0.0,
                    scale: 0.0,
                },
                players: Vec::new(),
            })
            .unwrap(),
        )
        .unwrap();
    }
}
