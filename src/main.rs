use iced::{Sandbox, Settings};
mod counter;
mod message;
use counter::Counter;
use message::Message;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
