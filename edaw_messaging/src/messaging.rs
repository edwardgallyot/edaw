mod message_queue;
mod packet_types;

pub use message_queue::*;
pub use packet_types::*;

pub trait MessageHandler {
    fn handle_message(message: &Message);
}
