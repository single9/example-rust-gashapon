use std::time::{self, UNIX_EPOCH};

use iced::widget::{button, column, container, row, scrollable, text, text_editor, text_input};
use iced::{Element, padding};
use test_gashapon::Item;

#[derive(Debug, Clone)]
pub enum Message {
    AddNewItemName(text_editor::Action),
    AddNewItemCount(text_editor::Action),
    InesertPrize,
    LockItems,
    RemovePrize(usize),
    Draw,
    UpdateUnitPrice(String),
    Clear,
}

#[derive(Debug, Clone)]
struct Prizes {
    pub temp_prize: text_editor::Content,
    pub temp_count: text_editor::Content,
    pub items: Vec<Item>,
    pub draw_rate: Vec<(Item, f64)>,
    pub drawed_items: Vec<String>,
}

impl Default for Prizes {
    fn default() -> Self {
        Self {
            temp_prize: text_editor::Content::new(),
            temp_count: text_editor::Content::new(),
            items: Vec::new(),
            draw_rate: Vec::new(),
            drawed_items: Vec::new(),
        }
    }
}

#[derive(Default)]
struct App {
    pub temp_unit_price: u64,
    pub unit_price: u64,
    pub prizes: Prizes,
    is_locked: bool,
    prize_pool: Vec<String>,
    idx_box: Vec<usize>,
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let lock_btn_text = if self.is_locked { "Unlock" } else { "Lock" };
        let lock_btn = button(lock_btn_text).on_press(Message::LockItems);
        let clear_btn = button("Clear").on_press(Message::Clear);

        scrollable(container(row![
            column![
                row![
                    column![
                        text("Gashapon Machine").size(50),
                        row![
                            text("Unit Price").size(20),
                            column![
                                text_input("Number", &self.temp_unit_price.to_string())
                                    .width(50)
                                    .on_input(Message::UpdateUnitPrice)
                            ]
                            .padding(padding::left(10))
                        ]
                    ]
                    .spacing(10)
                    .align_x(iced::Alignment::Start),
                ]
                .spacing(10)
                .padding(padding::all(20)),
                row![
                    column![
                        row![text("Prizes").size(20), lock_btn, clear_btn]
                            .align_y(iced::Alignment::Center),
                        column(self.prizes.items.iter().enumerate().map(|(idx, item)| {
                            row![
                                column![button("x").on_press(Message::RemovePrize(idx))]
                                    .padding(padding::right(10))
                                    .spacing(10),
                                column![
                                    text(format!("{}: {}", item.name, item.count))
                                        .shaping(text::Shaping::Advanced)
                                        .size(20),
                                ]
                                .spacing(10),
                            ]
                            .align_y(iced::Alignment::Center)
                            .into()
                        }))
                        .spacing(10),
                    ]
                    .spacing(10)
                    .padding(padding::all(20))
                    .align_x(iced::Alignment::Start),
                    column![
                        text("Draw Rate:").size(20),
                        column(self.prizes.draw_rate.iter().map(|(item, rate)| {
                            text(format!("{}: {:.2}%", item.name, rate * 100.0))
                                .shaping(text::Shaping::Advanced)
                                .size(20)
                                .into()
                        }))
                        .spacing(10),
                    ]
                    .padding(padding::all(20))
                    .spacing(10)
                    .align_x(iced::Alignment::Start)
                ]
                .spacing(10),
                row![column![
                    row![text("Pool:").size(20),],
                    text(format!(
                        "{}",
                        self.prize_pool
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .shaping(text::Shaping::Advanced)
                    .size(14),
                    row![text("Drawed Items:").size(20),],
                    text(format!(
                        "{}",
                        self.prizes
                            .drawed_items
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .shaping(text::Shaping::Advanced)
                    .size(14),
                ],]
                .padding(padding::all(20)),
                row![button("Draw").on_press(Message::Draw)].padding(padding::all(20)),
                row![
                    column![
                        row![text("Add New Prize").size(20),],
                        row![
                            text_editor(&self.prizes.temp_prize)
                                .placeholder("Name")
                                .on_action(Message::AddNewItemName)
                                .width(100)
                                .size(20),
                            text_editor(&self.prizes.temp_count)
                                .placeholder("Amount")
                                .on_action(Message::AddNewItemCount)
                                .width(100)
                                .size(20),
                            button("Insert Prize").on_press(Message::InesertPrize),
                        ]
                        .spacing(10)
                        .align_y(iced::Alignment::Center),
                    ]
                    .spacing(10)
                    .padding(padding::all(20))
                    .align_x(iced::Alignment::Start),
                ]
                .spacing(10),
            ]
            .padding(padding::all(20))
            .align_x(iced::Alignment::Start),
        ]))
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddNewItemName(action) => {
                self.prizes.temp_prize.perform(action);
            }
            Message::AddNewItemCount(action) => {
                self.prizes.temp_count.perform(action);
            }
            Message::InesertPrize => {
                if self.is_locked {
                    // Handle locked state
                    println!("Items are locked");
                    return;
                }
                let prize_name = if self.prizes.temp_prize.text().len() > 0 {
                    self.prizes.temp_prize.text().to_string()
                } else {
                    // Handle empty name input
                    println!("Empty name input");
                    return;
                };

                // Handle insert prize action here
                if let Some(count) = self.prizes.temp_count.text().parse::<i64>().ok() {
                    if count > 0 {
                        let item = Item::new(prize_name, count);
                        self.prizes.items.push(item);
                        self.prizes.temp_prize = text_editor::Content::new();
                        self.prizes.temp_count = text_editor::Content::new();
                        // Update the prizes after inserting a new item
                        self.update_prizes();
                    }
                } else {
                    // Handle invalid count input
                    println!("Invalid count input");
                }
            }
            Message::LockItems => {
                // Handle lock items action here
                self.is_locked = !self.is_locked;
                if self.is_locked {
                    // Lock the items
                    println!("Items are locked");
                } else {
                    // Unlock the items
                    println!("Items are unlocked");
                }
            }
            Message::RemovePrize(idx) => {
                if self.is_locked {
                    // Handle locked state
                    println!("Items are locked");
                    return;
                }
                // Handle remove prize action here
                if idx < self.prizes.items.len() {
                    self.prizes.items.remove(idx);
                    // Update the prizes after removing an item
                    self.update_prizes();
                }
            }
            Message::Draw => {
                if self.idx_box.len() == 0 {
                    println!("No more items left to draw.");
                    return;
                }

                let drawed_item =
                    test_gashapon::draw_prize_item(&mut self.idx_box, &mut self.prize_pool, None);
                self.prizes.drawed_items.push(drawed_item.clone());

                self.prizes
                    .items
                    .iter_mut()
                    .find(|item| item.name == drawed_item)
                    .map(|item| item.count -= 1);

                self.prizes.draw_rate = test_gashapon::calculate_draw_rate(&self.prizes.items)
                    .iter()
                    .map(|&(i, r)| (i.clone(), r))
                    .collect::<Vec<_>>();
            }
            Message::UpdateUnitPrice(price) => {
                // Handle update unit price action here
                if let Ok(price) = price.parse::<u64>() {
                    self.temp_unit_price = price;
                    // Update the unit price here
                    // This can be used to recalculate draw rates or other logic
                    self.unit_price = price;
                } else {
                    // Handle invalid unit price input
                    println!("Invalid unit price input");
                }
            }
            Message::Clear => {
                // Handle clear action here
                self.prizes.items.clear();
                self.prizes.drawed_items.clear();
                self.prizes.temp_prize = text_editor::Content::new();
                self.prizes.temp_count = text_editor::Content::new();
                self.prize_pool.clear();
                self.idx_box.clear();
                self.prizes.draw_rate.clear();
                self.is_locked = false;
                // Reset the unit price
                self.temp_unit_price = 0;
                self.unit_price = 0;
            }
        }
    }

    fn update_prizes(&mut self) {
        // Update the prizes here
        // This function can be used to recalculate draw rates or other logic
        self.prizes.draw_rate = test_gashapon::calculate_draw_rate(&self.prizes.items)
            .iter()
            .map(|&(i, r)| (i.clone(), r))
            .collect::<Vec<_>>();

        let items = {
            let mut items = Vec::new();
            for i in self.prizes.items.clone().into_iter() {
                let item = i.name;
                let mut item_count = i.count;
                while item_count > 0 {
                    items.push(item.to_string());
                    item_count -= 1;
                }
            }
            items
        };
        // Randomly sort the items
        let random_sort_items = {
            let mut seed: usize = time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;
            test_gashapon::random_sort(&items, &mut seed)
        };

        self.prize_pool = random_sort_items.clone();
        let count = self.prizes.items.iter().map(|item| item.count).sum::<i64>();
        let prize_item_count = count as usize;

        let idx_box = {
            let mut idx_box = Vec::new();
            for i in 0..prize_item_count {
                idx_box.push(i);
            }
            let mut seed: usize = time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;
            test_gashapon::random_sort(&idx_box, &mut seed)
        };

        self.idx_box = idx_box.clone();
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Gashapon GUI")
        .window_size((800.0, 800.0))
        .run()
}
