use iced::widget::{Column, button, column, container, rich_text, row, span, text, text_editor};
use iced::{Element, Font, color, font, padding};
use test_gashapon::Item;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    Edit(text_editor::Action),
}

#[derive(Debug, Clone)]
pub enum PrizeItem {
    Insert,
    Remove,
    Draw,
}

#[derive(Debug, Clone)]
struct Prizes {
    items: Vec<Item>,
    draw_rate: Vec<(Item, f64)>,
    drawed_items: Vec<String>,
}

impl Default for Prizes {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            draw_rate: Vec::new(),
            drawed_items: Vec::new(),
        }
    }
}

#[derive(Default)]
struct App {
    unit_price: u32,
    prizes: Prizes,
}

#[derive(Default)]
struct Counter {
    value: i64,
    text: text_editor::Content,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
            text_editor(&self.text)
                .placeholder("Type something here...")
                .on_action(Message::Edit)
                .size(50)
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Edit(action) => {
                self.text.perform(action);
            }
        }
    }
}

// fn view(counter: &Counter) -> Element<Message> {
//     container(
//         row![
//             column![
//                 text(counter.value).size(20),
//                 button("Increment").on_press(Message::Increment),
//             ]
//             .spacing(10),
//             column![text("Hello, world!").size(20),].spacing(10),
//         ]
//         .spacing(10),
//     )
//     .padding(padding::all(20))
//     .center(800)
//     // .style(container::rounded_box)
//     .into()
// }

// fn update(counter: &mut Counter, message: Message) {
//     match message {
//         Message::Increment => counter.value += 1,
//     }
// }

pub fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Gashapon GUI")
        .run()
}
