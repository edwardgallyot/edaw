mod message_rx;
mod message_tx;

pub use message_rx::MessageRx;
pub use message_tx::MessageTx;

use crossbeam_channel::bounded;

pub struct MessageQueue {
    message_tx: Option<MessageTx>,
    message_rx: Option<MessageRx>,
}

impl MessageQueue {
    pub fn new() -> MessageQueue {
        let (tx, rx) = bounded(32);

        let message_tx = Some(MessageTx::new(tx));
        let message_rx = Some(MessageRx::new(rx));

        MessageQueue {
            message_tx,
            message_rx,
        }
    }

    pub fn take_tx(&mut self) -> Option<MessageTx> {
        self.message_tx.take()
    }

    pub fn take_rx(&mut self) -> Option<MessageRx> {
        self.message_rx.take()
    }
}
