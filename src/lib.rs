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
    use super::*;
    use bincode::{deserialize, serialize};
    use client_frame::Movement;

    #[test]
    fn test_client_message_serialize() -> () {
        serialize(&ClientFrame {
            messages: vec![client_frame::ChatMessage {
                receiver: client_frame::ChatReceiver::Broadcast,
                text: String::from("Hello, world!"),
            }],
            movement: Movement::default(),
            rotation: 0.0
        })
        .unwrap();

        serialize(&ClientFrame {
            messages: vec![client_frame::ChatMessage {
                receiver: client_frame::ChatReceiver::Direct {
                    target: String::from("hunter2"),
                },
                text: String::from("Hello, world!"),
            }],
            movement: Movement::default(),
            rotation: 0.0
        })
        .unwrap();
    }

    #[test]
    fn test_client_message_deserialize() -> () {
        deserialize::<ClientFrame>(
            &serialize(&ClientFrame {
                messages: vec![client_frame::ChatMessage {
                    receiver: client_frame::ChatReceiver::Broadcast,
                    text: String::from("Hello, world!"),
                }],
                movement: Movement::default(),
                rotation: 0.0
            })
            .unwrap(),
        )
        .unwrap();

        deserialize::<ClientFrame>(
            &serialize(&ClientFrame {
                messages: vec![client_frame::ChatMessage {
                    receiver: client_frame::ChatReceiver::Direct {
                        target: String::from("hunter2"),
                    },
                    text: String::from("Hello, world!"),
                }],
                movement: Movement::default(),
                rotation: 0.0
            })
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_server_message_serialize() -> () {
        serialize(&ServerFrame {
            messages: vec![server_frame::ChatMessage {
                kind: server_frame::ChatMessageKind::Broadcast,
                sender: String::from("hunter1"),
                text: String::from("Hello, world!"),
            }],
            position: [0.0, 0.0, 0.0],
            rotation: 0.0,
        })
        .unwrap();
    }

    #[test]
    fn test_server_message_deserialize() -> () {
        deserialize::<ServerFrame>(
            &serialize(&ServerFrame {
                messages: vec![server_frame::ChatMessage {
                    kind: server_frame::ChatMessageKind::Broadcast,
                    sender: String::from("hunter1"),
                    text: String::from("Hello, world!"),
                }],
                position: [0.0, 0.0, 0.0],
                rotation: 0.0,
            })
            .unwrap(),
        )
        .unwrap();
    }
}
