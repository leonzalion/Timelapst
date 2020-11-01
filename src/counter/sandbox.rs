use iced::{Sandbox, Element};
use super::Counter;
use crate::Message;

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
       self.handle_update(message)
    }

    fn view(&mut self) -> Element<Message> {
        self.handle_view()
    }
}
