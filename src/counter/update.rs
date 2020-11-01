mod update {
    use crate::message::Message;
    use crate::Counter;

    impl Counter {
        pub fn handle_update(&mut self, message: Message) {
            match message {
                Message::IncrementPressed => {
                    self.value += 1;
                }
                Message::DecrementPressed => {
                    self.value -= 1;
                }
            }
        }
    }
}
