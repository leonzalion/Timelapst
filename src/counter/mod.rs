use iced::button;

mod sandbox;
mod view;
mod update;

#[derive(Default)]
pub struct Counter {
    pub value: i32,
    pub increment_button: button::State,
    pub decrement_button: button::State,
}
