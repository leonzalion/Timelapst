mod view {
    use iced::{Align, Button, Column, Text, Element};
    use crate::message::Message;
    use crate::Counter;

    impl Counter {
        pub fn handle_view(&mut self) -> Element<Message> {
            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.increment_button, Text::new("Increment"))
                        .on_press(Message::IncrementPressed),
                )
                .push(Text::new(self.value.to_string()).size(50))
                .push(
                    Button::new(&mut self.decrement_button, Text::new("Decrement"))
                        .on_press(Message::DecrementPressed),
                )
                .into()
        }
    }
}
