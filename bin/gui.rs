use iced::Element;
use iced::widget::{button, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn view(counter: &Counter) -> Element<Message> {
    button(text(counter.value))
        .on_press(Message::Increment)
        .into()
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}
