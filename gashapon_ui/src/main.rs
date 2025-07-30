use dioxus::{logger::tracing, prelude::*};
use gashapon::{Gashapon, GashaponItem, PrizeItem};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone)]
pub struct Prizes {
    pub temp_prize: String,
    pub temp_count: u64,
    pub draw_rate: Vec<(PrizeItem, f64)>,
    pub drawed_items: Vec<PrizeItem>,
}

impl Default for Prizes {
    fn default() -> Self {
        Self {
            temp_prize: String::new(),
            temp_count: 0,
            draw_rate: Vec::new(),
            drawed_items: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Data {
    pub unit_price: Signal<u64>,
    pub prizes: Signal<Prizes>,
    pub draw_times: Signal<u64>,
    pub gashapon: Signal<Gashapon>,
    pub is_locked: Signal<bool>,
    pub prize_pool: Signal<Vec<Option<PrizeItem>>>,
    pub total_price_in_pool: Signal<u64>,
    pub current_cost: Signal<u64>,
}

impl Data {
    pub fn update_prizes(&mut self) {
        self.prizes.write().draw_rate = self
            .gashapon
            .read()
            .calculate_draw_rate()
            .into_iter()
            .map(|(i, r)| (i.clone().prize, r))
            .collect::<Vec<_>>();
        let new_prize = self
            .gashapon
            .read()
            .prizes
            .get_randomized_items()
            .into_iter()
            .map(|x| x.cloned())
            .collect::<Vec<_>>();
        self.prize_pool.set(new_prize);
    }

    pub fn update_price(&mut self) {
        self.total_price_in_pool.set(
            self.gashapon
                .read()
                .items
                .iter()
                .map(|(_, i)| i.quantity * self.unit_price.read().clone())
                .sum::<u64>(),
        );

        self.current_cost
            .set((self.prizes.read().drawed_items.len() as u64) * self.unit_price.read().clone());
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize the Gashapon with default items
    use_context_provider(|| Data::default());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title { "Gashapon Simulator" }
        AppLayout {
            h1 { "Gashapon Simulator" }
            UnitPrice {}
            PrizeList {}
            Pool {}
            FunctionButtons {}
            DrawnItems {}
        }
    }
}

#[component]
pub fn AppLayout(children: Element) -> Element {
    rsx! {
        div { id: "app",
            div { id: "header" }
            div { id: "content", {children} }
            footer { id: "footer",
                p {
                    "© 2025 Duye Chen "
                    a { href: "https://github.com/single9/example-rust-gashapon",
                        "GitHub"
                    }
                    " | "
                    a { href: "https://single9.net", "Website" }
                }
            }
        }
    }
}

#[component]
pub fn UnitPrice() -> Element {
    let mut data = use_context::<Data>();
    rsx! {
        div { id: "unit-price",
            label { r#for: "unit-price-input", "Unit Price: " }
            input {
                id: "unit-price-input",
                placeholder: "Price",
                width: "100px",
                oninput: move |e| {
                    let value = e.value().parse::<u64>().unwrap_or(0);
                    data.unit_price.set(value);
                    data.update_price();
                    tracing::debug!("Unit price set to: {}", data.unit_price);
                },
            }
        }
        div { id: "total-price",
            label { "Total Price: " }
            span { "{data.total_price_in_pool}" }
        }
        div { id: "current-cost",
            label { "Current Cost: " }
            span { "{data.current_cost}" }
        }
    }
}

#[component]
pub fn PrizeList() -> Element {
    let mut data = use_context::<Data>();

    rsx! {
        div { id: "prize-list",
            h2 { "Prizes" }
            div { id: "prize-inputs",
                label { r#for: "prize-name-input", "Prize Name: " }
                input {
                    id: "prize-name-input",
                    placeholder: "Prize Name",
                    value: "{data.prizes.read().temp_prize}",
                    oninput: move |e| {
                        data.prizes.write().temp_prize = e.value();
                    },
                }
                label { r#for: "prize-count-input", "Count: " }
                input {
                    id: "prize-count-input",
                    r#type: "number",
                    value: "{data.prizes.read().temp_count}",
                    oninput: move |e| {
                        data.prizes.write().temp_count = e.value().parse().unwrap_or(0);
                    },
                }
                button {
                    onclick: move |_| {
                        let mut data = use_context::<Data>();
                        let prize_name = data.prizes.read().temp_prize.clone();
                        let prize_count = data.prizes.read().temp_count.clone();
                        tracing::debug!("Adding prize: {}, Count: {}", prize_name, prize_count);
                        if !prize_name.is_empty() && prize_count > 0 {
                            let prize_item = PrizeItem::new(prize_name.clone());
                            let gashapon_item = GashaponItem::new(prize_item)
                                .with_quantity(prize_count.clone());
                            data.gashapon.write().add_item(gashapon_item);
                            data.gashapon.write().build();
                            data.prizes.write().temp_prize = String::new();
                            data.prizes.write().temp_count = 0;
                            data.update_prizes();
                            data.update_price();
                            tracing::debug!("Prize added: {:?}", data.prize_pool);
                        } else {
                            tracing::warn!("Prize name or count is invalid.");
                        }
                        tracing::debug!("Current prize pool: {:?}", data.gashapon.read());
                    },
                    "Add"
                }
            }
            div { id: "prize-items",
                h3 { "Current Prizes" }
                ul { class: "prize-items",
                    for (_ , item) in data.gashapon.read().items.iter() {
                        li { "{item.prize.name} ({item.quantity})" }
                    }
                }
            }

            div { id: "draw-rates",
                h3 { "Draw Rates" }
                ul { class: "prize-items",
                    for (item , rate) in data.prizes.read().draw_rate.iter() {
                        li { "{item.name}: {rate * 100.0:.2}%" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Pool() -> Element {
    let data = use_context::<Data>();
    let mut display_prize_pool = use_signal(|| false);
    rsx! {
        div { id: "pool-items",
            h3 {
                onclick: move |_| {
                    let x = display_prize_pool.read().clone();
                    display_prize_pool.set(!x);
                },
                "Pool "
                span {
                    class: "toggle-icon",
                    style: "cursor: pointer;font-size: 0.75em;",
                    if display_prize_pool.read().clone() {
                        "▲"
                    } else {
                        "▼"
                    }
                }
            }
            span {
                "Total Items in Pool: {data.prize_pool.read().len() - data.prizes.read().drawed_items.len()}"
            }
            div { id: "show-prize-pool",
                if display_prize_pool.read().clone() {
                    ul { class: "prize-items",
                        for item in data.prize_pool
                            .read()
                            .iter()
                            .map(|i| match i {
                                Some(prize) => prize.name.clone(),
                                None => "-".to_string(),
                            })
                            .collect::<Vec<_>>()
                        {
                            li { "{item}" }
                        }
                    }
                }
                hr {}
            }
        }
    }
}

#[component]
pub fn DrawButton() -> Element {
    let mut data = use_context::<Data>();
    rsx! {
        button {
            class: "mr-5",
            onclick: move |_| {
                if data.gashapon.read().prizes.idx_box.is_empty() {
                    tracing::warn!("No more items left to draw.");
                    return;
                }
                let my_prize = data.gashapon.write().draw();
                data.prizes.write().drawed_items.push(my_prize.clone());
                tracing::debug!("Drawn prize: {:?}", my_prize);
                data.update_prizes();
                data.update_price();
            },
            "Draw Prize"
        }
    }
}

#[component]
pub fn RestoreButton() -> Element {
    let mut data = use_context::<Data>();
    rsx! {
        button {
            class: "mr-5",
            onclick: move |_| {
                data.gashapon.write().restore_items();
                data.prizes.write().drawed_items.clear();
                data.update_prizes();
                data.update_price();
                tracing::debug!("Items restored to original state.");
            },
            "Restore Items"
        }
    }
}

#[component]
pub fn ClearButton() -> Element {
    let mut data = use_context::<Data>();
    rsx! {
        button {
            class: "mr-5 btn-error",
            onclick: move |_| {
                data.gashapon.write().items.clear();
                data.gashapon.write().prizes.items.clear();
                data.gashapon.write().prizes.randomized_items.clear();
                data.prizes.write().drawed_items.clear();
                data.prize_pool.write().clear();
                data.prizes.write().draw_rate.clear();
                data.total_price_in_pool.set(0);
                data.current_cost.set(0);
                tracing::debug!("All items cleared.");
            },
            "Clear All Items"
        }
    }
}

#[component]
pub fn DrawnItems() -> Element {
    let data = use_context::<Data>();
    rsx! {
        div { id: "drawn-items",
            h3 { "Drawn Items" }
            ul { class: "prize-items",
                for item in data.prizes.read().drawed_items.iter() {
                    li { "{item.name}" }
                }
            }
        }
    }
}

#[component]
pub fn FunctionButtons() -> Element {
    rsx! {
        div { id: "function-buttons",
            DrawButton {}
            RestoreButton {}
            ClearButton {}
        }
    }
}
