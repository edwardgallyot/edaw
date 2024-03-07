mod connection_action;
pub use connection_action::*;

#[derive(Debug)]
pub enum Screen {
    Main,
    Connection,
    Samples,
}
