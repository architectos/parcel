pub use bincode;
pub use serde;

#[cfg(feature = "json-schema")]
pub use schemars;

pub mod client_message;
pub use client_message::ClientMessage;

pub mod server_message;
pub use server_message::ServerMessage;

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{deserialize, serialize};

    #[test]
    fn test_client_message_serialize() -> () {
        serialize(&ClientMessage::Chat {
            receiver: client_message::ChatReceiver::All,
            text: String::from("Hello, world!"),
        })
        .unwrap();

        serialize(&ClientMessage::Chat {
            receiver: client_message::ChatReceiver::Direct {
                target: String::from("hunter1"),
            },
            text: String::from("Hello, world!"),
        })
        .unwrap();

        serialize(&ClientMessage::Chat {
            receiver: client_message::ChatReceiver::Group { id: 0 },
            text: String::from("Hello, world!"),
        })
        .unwrap();
    }

    #[test]
    fn test_client_message_deserialize() -> () {
        deserialize::<ClientMessage>(
            &serialize(&ClientMessage::Chat {
                receiver: client_message::ChatReceiver::All,
                text: String::from("Hello, world!"),
            })
            .unwrap(),
        )
        .unwrap();

        deserialize::<ClientMessage>(
            &serialize(&ClientMessage::Chat {
                receiver: client_message::ChatReceiver::Direct {
                    target: String::from("hunter1"),
                },
                text: String::from("Hello, world!"),
            })
            .unwrap(),
        )
        .unwrap();

        deserialize::<ClientMessage>(
            &serialize(&ClientMessage::Chat {
                receiver: client_message::ChatReceiver::Group { id: 0 },
                text: String::from("Hello, world!"),
            })
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_server_message_serialize() -> () {
        serialize(&ServerMessage::Chat {
            sender: String::from("hunter1"),
            text: String::from("Hello, world!"),
        })
        .unwrap();

        serialize(&ServerMessage::Empty {}).unwrap();
    }

    #[test]
    fn test_server_message_deserialize() -> () {
        deserialize::<ServerMessage>(
            &serialize(&ServerMessage::Chat {
                sender: String::from("hunter1"),
                text: String::from("Hello, world!"),
            })
            .unwrap(),
        )
        .unwrap();

        deserialize::<ServerMessage>(&serialize(&ServerMessage::Empty {}).unwrap()).unwrap();
    }
}
