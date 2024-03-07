mod packet_types;
mod message_queue;

pub use packet_types::*;
pub use message_queue::*;

pub trait MessageHandler {
    fn handle_message(message: &Message);
}
