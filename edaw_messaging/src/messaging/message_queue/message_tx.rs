use anyhow::{anyhow, Result};
use crossbeam_channel::Sender;

use crate::Message;

pub struct MessageTx {
    producer: Sender<Message>,
}

impl MessageTx {
    pub fn new(tx: Sender<Message>) -> MessageTx {
        let producer = tx;
        MessageTx { producer }
    }

    pub fn send(&mut self, message: Message) -> Result<()> {
        match self.producer.try_send(message) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("error sending message: {}", e)),
        }
    }
}
