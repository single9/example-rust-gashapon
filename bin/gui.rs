use iced::widget::{button, column, container, row, scrollable, text, text_editor, text_input};
use iced::{Element, padding};
use test_gashapon::{Gashapon, GashaponItem, PrizeId, PrizeItem};

#[derive(Debug, Clone)]
pub enum Message {
    AddNewItemName(text_editor::Action),
    AddNewItemCount(text_editor::Action),
    InesertPrize,
    LockItems,
    RemoveItem(PrizeId),
    IncrementItem(PrizeId),
    ReduceItem(PrizeId),
    Draw,
    UpdateUnitPrice(String),
    Clear,
    Restore,
}

#[derive(Debug, Clone)]
struct Prizes {
    pub temp_prize: text_editor::Content,
    pub temp_count: text_editor::Content,
    pub draw_rate: Vec<(PrizeItem, f64)>,
    pub drawed_items: Vec<PrizeItem>,
}

impl Default for Prizes {
    fn default() -> Self {
        Self {
            temp_prize: text_editor::Content::new(),
            temp_count: text_editor::Content::new(),
            draw_rate: Vec::new(),
            drawed_items: Vec::new(),
        }
    }
}

#[derive(Default)]
struct App {
    pub temp_unit_price: u64,
    pub unit_price: u64,
    pub total_price_in_pool: u64,
    pub current_cost: u64,
    pub prizes: Prizes,
    gashapon: Gashapon,
    is_locked: bool,
    prize_pool: Vec<Option<PrizeItem>>,
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let lock_btn_text = if self.is_locked { "Unlock" } else { "Lock" };
        let lock_btn = button(lock_btn_text)
            .style(iced::widget::button::success)
            .on_press_maybe(if self.gashapon.items.is_empty() {
                None
            } else {
                Some(Message::LockItems)
            });
        let restore_btn = button("Restore")
            .style(iced::widget::button::secondary)
            .on_press_maybe(if self.is_locked || self.gashapon.items.is_empty() {
                None
            } else {
                Some(Message::Restore)
            });
        let clear_btn = button("Clear")
            .style(iced::widget::button::danger)
            .on_press_maybe(if self.is_locked || self.gashapon.items.is_empty() {
                None
            } else {
                Some(Message::Clear)
            });
        let draw_btn = button("Draw").on_press_maybe(if self.is_locked {
            Some(Message::Draw)
        } else {
            None
        });

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
                            .padding(padding::left(10)),
                        ],
                        row![
                            column![
                                text(format!("Total Price: {}", self.total_price_in_pool)).size(20),
                            ],
                            column![text(format!("Current Cost: {}", self.current_cost)).size(20),]
                                .padding(padding::left(10))
                        ]
                    ]
                    .spacing(10)
                    .align_x(iced::Alignment::Start),
                ]
                .spacing(10)
                .padding(padding::all(20)),
                row![lock_btn, restore_btn, clear_btn]
                    .padding(padding::left(20))
                    .align_y(iced::Alignment::End),
                row![
                    column![
                        row![text("Prizes").size(20)].align_y(iced::Alignment::Center),
                        column(self.gashapon.items.values().map(|item| {
                            row![
                                row![
                                    column![
                                        button("x")
                                            .style(button::text)
                                            .on_press(Message::RemoveItem(item.get_prize_id())),
                                    ]
                                    .align_x(iced::Alignment::Center)
                                    .padding(iced::padding::right(10)),
                                    column![
                                        text(format!("{}: {}", item.prize.name, item.quantity))
                                            .shaping(text::Shaping::Advanced)
                                            .size(20),
                                    ],
                                ]
                                .align_y(iced::Alignment::Center),
                                row![
                                    button("+")
                                        .style(button::text)
                                        .on_press(Message::IncrementItem(item.get_prize_id())),
                                    button("-")
                                        .style(button::text)
                                        .on_press(Message::ReduceItem(item.get_prize_id()))
                                ]
                                .align_y(iced::Alignment::Center)
                            ]
                            .into()
                        }))
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
                            .map(|s| {
                                match s {
                                    Some(d) => d.name.to_string(),
                                    _ => "-".to_string(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .shaping(text::Shaping::Advanced)
                    .size(14),
                    row![
                        text(format!(
                            "Drawed Items: [{}]",
                            self.prizes.drawed_items.len()
                        ))
                        .size(20),
                    ],
                    text(format!(
                        "{}",
                        self.prizes
                            .drawed_items
                            .iter()
                            .map(|s| s.name.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .shaping(text::Shaping::Advanced)
                    .size(14),
                ],]
                .padding(padding::all(20)),
                row![draw_btn].padding(padding::all(20)),
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
                if let Some(count) = self.prizes.temp_count.text().parse::<u64>().ok() {
                    if count > 0 {
                        let item =
                            GashaponItem::new(PrizeItem::new(prize_name)).with_quantity(count);
                        self.gashapon.add_item(item);
                        self.gashapon.build();
                        self.prizes.temp_prize = text_editor::Content::new();
                        self.prizes.temp_count = text_editor::Content::new();
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
            Message::RemoveItem(id) => {
                if self.is_locked {
                    // Handle locked state
                    println!("Items are locked");
                    return;
                }
                self.gashapon.remove_item(id);
            }
            Message::IncrementItem(id) => {
                if self.is_locked {
                    // Handle locked state
                    println!("Items are locked");
                    return;
                }

                let quantity = self.gashapon.items.get(&id).unwrap().quantity;
                self.gashapon.update_item_quantity(id, quantity + 1);
            }
            Message::ReduceItem(id) => {
                if self.is_locked {
                    // Handle locked state
                    println!("Items are locked");
                    return;
                }

                let quantity = self.gashapon.items.get(&id).unwrap().quantity;

                if quantity - 1 > 0 {
                    self.gashapon.update_item_quantity(id, quantity - 1);
                }
            }
            Message::Draw => {
                if self.gashapon.prizes.idx_box.len() == 0 {
                    println!("No more items left to draw.");
                    return;
                }

                let drawed_item = self.gashapon.draw();
                self.prizes.drawed_items.push(drawed_item.clone());
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
                self.prizes.drawed_items.clear();
                self.prizes.temp_prize = text_editor::Content::new();
                self.prizes.temp_count = text_editor::Content::new();
                self.prizes.draw_rate.clear();
                self.is_locked = false;
                // Reset the unit price
                self.temp_unit_price = 0;
                self.gashapon.items.clear();
                self.gashapon.prizes.randomized_items.clear();
                self.gashapon.prizes.items.clear();
                self.prize_pool.clear();
            }
            Message::Restore => {
                self.gashapon.restore_items();
                self.prizes.drawed_items.clear();
            }
        }

        self.update_prizes();
        self.update_price();
    }

    fn update_price(&mut self) {
        self.total_price_in_pool = self
            .gashapon
            .items
            .iter()
            .map(|(_, i)| i.quantity * self.unit_price)
            .sum::<u64>();

        self.current_cost = (self.prizes.drawed_items.len() as u64) * self.unit_price;
    }

    fn update_draw_rate(&mut self) {
        self.prizes.draw_rate = self
            .gashapon
            .calculate_draw_rate()
            .into_iter()
            .map(|(i, r)| (i.clone().prize, r))
            .collect::<Vec<_>>();
    }

    fn update_prizes(&mut self) {
        // Update the prizes here
        // This function can be used to recalculate draw rates or other logic
        self.update_draw_rate();
        self.prize_pool = self
            .gashapon
            .prizes
            .get_randomized_items()
            .into_iter()
            .map(|x| x.cloned())
            .collect();
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Gashapon GUI")
        .window_size((800.0, 800.0))
        .run()
}
