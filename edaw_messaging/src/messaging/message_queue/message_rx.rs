use anyhow::{anyhow, Result};
use crossbeam::channel::{Receiver, TryRecvError};

use crate::Message;

pub struct MessageRx {
    consumer: Receiver<Message>,
}

impl MessageRx {
    pub fn new(rx: Receiver<Message>) -> MessageRx {
        let consumer = rx;
        MessageRx { consumer }
    }

    pub fn recv(&mut self) -> Result<Option<Message>> {
        match self.consumer.try_recv() {
            Ok(m) => Ok(Some(m)),
            Err(e) if e == TryRecvError::Empty => Ok(None),
            Err(e) => Err(anyhow!("error receiving from channel {}", e)),
        }
    }
}
