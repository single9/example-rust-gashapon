mod utils;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{self, UNIX_EPOCH};

use utils::randomize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrizeId(u64);

impl PrizeId {
    pub fn new<T>(name: T) -> Self
    where
        T: ToString,
    {
        let name = name.to_string();
        let mut hasher = DefaultHasher::new();

        name.hash(&mut hasher);

        Self(hasher.finish())
    }

    pub fn get_id(&self) -> u64 {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct GashaponItem {
    pub prize: PrizeItem,
    pub quantity: u64,
    original_quantity: u64,
}

impl GashaponItem {
    pub fn new(prize: PrizeItem) -> Self {
        Self {
            prize,
            quantity: u64::default(),
            original_quantity: u64::default(),
        }
    }

    pub fn with_quantity(mut self, quantity: u64) -> Self {
        self.quantity = quantity;
        self.original_quantity = quantity;
        self
    }

    pub fn get_prize_id(&self) -> PrizeId {
        self.prize.get_id()
    }

    pub fn restore(&mut self) {
        self.quantity = self.original_quantity;
    }
}

pub trait GetPrizeItemId {
    fn get_id(&self) -> PrizeId;
}

#[derive(Debug, Clone)]
pub struct PrizeItem {
    id: PrizeId,
    pub name: String,
}

impl PrizeItem {
    pub fn new<T>(name: T) -> Self
    where
        T: ToString + Clone,
    {
        Self {
            id: PrizeId::new(name.clone()),
            name: name.to_string(),
        }
    }
}

impl GetPrizeItemId for PrizeItem {
    fn get_id(&self) -> PrizeId {
        self.id.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Prizes {
    pub items: Vec<PrizeItem>,
    pub idx_box: Vec<usize>,
    pub randomized_items: Vec<Option<usize>>,
    seed: Option<usize>,
}

impl Prizes {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            idx_box: Vec::new(),
            randomized_items: Vec::new(),
            seed: None,
        }
    }

    fn randomnize_items(&mut self) {
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(idx, _)| Some(idx))
            .collect::<Vec<Option<usize>>>();
        let mut seed = self.get_seed();
        self.randomized_items = randomize(items, &mut seed);
    }

    fn update_idx_box(&mut self) {
        let idx_box = self
            .items
            .iter()
            .enumerate()
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        let mut seed = self.get_seed();
        self.idx_box = randomize(idx_box, &mut seed);
    }

    pub fn with_items(&mut self, items: Vec<&GashaponItem>) {
        let items = {
            let mut items_vec = Vec::new();
            for item in items {
                for _ in 0..item.quantity {
                    items_vec.push(item.prize.clone());
                }
            }
            items_vec
        };
        self.items = items.clone();
    }

    pub fn with_seed(&mut self, seed: usize) {
        self.seed = Some(seed);
    }

    /// Get the seed value, or generate a new one based on the current time if not set.
    pub fn get_seed(&self) -> usize {
        self.seed.unwrap_or_else(|| {
            time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
        })
    }

    pub fn build(&mut self) {
        self.randomnize_items();
        self.update_idx_box();
    }

    pub fn draw(&mut self) -> PrizeItem {
        if self.idx_box.is_empty() {
            panic!("No more items to draw!");
        }

        let idx = self.idx_box.pop().unwrap();
        let item_idx = self.randomized_items[idx].clone();

        self.randomized_items[idx] = None; // Mark as drawn
        self.get_item_by_index(item_idx)
            .cloned()
            .expect("Item already drawn")
    }

    pub fn get_item_by_index(&self, index: Option<usize>) -> Option<&PrizeItem> {
        let Some(idx) = index else {
            return None;
        };

        self.items.get(idx)
    }

    pub fn get_randomized_items(&self) -> Vec<Option<&PrizeItem>> {
        self.randomized_items
            .iter()
            .map(|idx| self.get_item_by_index(*idx))
            .collect::<Vec<_>>()
    }

    pub fn quantity(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug, Clone)]
pub struct Gashapon {
    pub items: HashMap<PrizeId, GashaponItem>,
    pub prizes: Prizes,
}

impl Default for Gashapon {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            prizes: Prizes::new(),
        }
    }
}

impl Gashapon {
    pub fn add_item(&mut self, item: GashaponItem) -> &mut Self {
        self.items.insert(item.get_prize_id(), item);
        self
    }

    pub fn add_items(&mut self, items: Vec<GashaponItem>) -> &mut Self {
        for item in items {
            self.items.insert(item.get_prize_id(), item);
        }
        self
    }

    pub fn restore_items(&mut self) -> &mut Self {
        for item in self.items.values_mut() {
            item.restore();
        }
        self.build()
    }

    pub fn with_seed(&mut self, seed: usize) -> &mut Self {
        self.prizes.with_seed(seed);
        self
    }

    pub fn remove_item(&mut self, id: PrizeId) -> &mut Self {
        self.items.remove(&id);
        self
    }

    pub fn build(&mut self) -> &mut Self {
        self.prizes
            .with_items(self.items.iter().map(|(_, item)| item).collect());
        self.prizes.build();
        self
    }

    pub fn draw(&mut self) -> PrizeItem {
        let prize = self.prizes.draw();
        let item = self.items.get_mut(&prize.get_id()).unwrap();
        item.quantity -= 1;
        prize
    }

    pub fn calculate_draw_rate(&self) -> Vec<(GashaponItem, f64)> {
        let mut draw_rate = Vec::new();
        // Calculate the total quantity of items
        let total_quantity = self
            .items
            .iter()
            .map(|(_, item)| item.quantity)
            .sum::<u64>() as f64;
        // Calculate the draw rate for each item
        // and store it in the HashMap
        for (_, item) in self.items.clone().into_iter() {
            let rate = ((item.quantity as f64) / total_quantity).max(0.0);
            draw_rate.push((item, rate));
        }
        draw_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prizes() {
        let mut prizes = Prizes::new();
        prizes.with_items(vec![
            &GashaponItem::new(PrizeItem::new("Item1")).with_quantity(2),
            &GashaponItem::new(PrizeItem::new("Item2")).with_quantity(3),
        ]);
        prizes.with_seed(12345);
        prizes.build();

        let drawn_item = prizes.draw();
        assert!(drawn_item.name == "Item1" || drawn_item.name == "Item2");
    }

    #[test]
    fn test_gashapon() {
        let mut gashapon = Gashapon::default();
        gashapon
            .add_item(GashaponItem::new(PrizeItem::new("Item1")).with_quantity(2))
            .add_item(GashaponItem::new(PrizeItem::new("Item2")).with_quantity(3))
            .with_seed(12345)
            .build();

        let drawn_item = gashapon.draw();
        assert!(drawn_item.name == "Item1" || drawn_item.name == "Item2");
    }

    #[test]
    fn test_gashapon_calculate_draw_rate() {
        let mut gashapon = Gashapon::default();
        gashapon.add_items(vec![
            GashaponItem::new(PrizeItem::new("Item1")).with_quantity(2),
            GashaponItem::new(PrizeItem::new("Item2")).with_quantity(3),
            GashaponItem::new(PrizeItem::new("Item3")).with_quantity(5),
        ]);
        gashapon.with_seed(12345).build();

        let draw_rate = gashapon.calculate_draw_rate();
        assert_eq!(draw_rate.len(), 3);
        assert!(
            draw_rate
                .iter()
                .map(|x| x.clone())
                .any(|(item, rate)| item.prize.name == "Item1" && rate > 0.0)
        );
    }

    #[test]
    fn test_gashapon_restore_items() {
        let mut gashapon = Gashapon::default();
        gashapon.add_items(vec![
            GashaponItem::new(PrizeItem::new("Item1")).with_quantity(1),
            GashaponItem::new(PrizeItem::new("Item2")).with_quantity(1),
        ]);
        gashapon.with_seed(12345).build();

        let drawn_item = gashapon.draw();
        assert!(drawn_item.name == "Item1" || drawn_item.name == "Item2");

        gashapon.restore_items();
        let draw_rate = gashapon.calculate_draw_rate();
        assert!(
            draw_rate
                .iter()
                .map(|x| x.clone())
                .any(|(item, rate)| item.quantity > 0 && rate > 0.0)
        );
    }
}
