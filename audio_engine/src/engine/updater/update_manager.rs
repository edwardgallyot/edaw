use anyhow::Result;
use edaw_messaging::MessageRx;

pub struct UpdateManager {
    rx: MessageRx,
}

impl UpdateManager {
    pub fn new(rx: MessageRx) -> UpdateManager {
        UpdateManager {
            rx          
        }
    }

    pub fn handle_updates(&mut self) -> Result<()> {
        match self.rx.recv() {
            Ok(Some(m))  => println!("message received: {:?}", m),
            Err(e) => eprintln!("error receiving: {}", e),
            Ok(None) => {},
        };
        Ok(())
    }
}
