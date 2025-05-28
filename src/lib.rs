mod utils;

use std::time::{self, UNIX_EPOCH};

use utils::randomize;

type PrizeCount = u64;
type GashaponItems = (PrizeItem, PrizeCount);

#[derive(Debug, Clone)]
pub struct PrizeItem {
    pub name: String,
}

impl PrizeItem {
    pub fn new<T>(name: T) -> Self
    where
        T: ToString,
    {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn with_items(&mut self, items: Vec<GashaponItems>) {
        let items = {
            let mut items_vec = Vec::new();
            for (item, count) in items {
                for _ in 0..count {
                    items_vec.push(item.clone());
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

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug, Clone)]
pub struct Gashapon {
    pub items: Vec<GashaponItems>,
    pub prizes: Prizes,
}

impl Gashapon {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            prizes: Prizes::new(),
        }
    }

    pub fn add_item<T>(&mut self, item: T, count: PrizeCount) -> &mut Self
    where
        T: ToString,
    {
        self.items.push((PrizeItem::new(item), count));
        self
    }

    pub fn add_items<T>(&mut self, items: Vec<(T, PrizeCount)>) -> &mut Self
    where
        T: ToString,
    {
        for (item, count) in items {
            self.items.push((PrizeItem::new(item), count));
        }
        self
    }

    pub fn with_items(&mut self, items: Vec<GashaponItems>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn with_prizes(&mut self, prizes: Prizes) -> &mut Self {
        self.prizes = prizes;
        self
    }

    pub fn with_seed(&mut self, seed: usize) -> &mut Self {
        self.prizes.with_seed(seed);
        self
    }

    pub fn build(&mut self) -> &mut Self {
        self.prizes.with_items(self.items.clone());
        self.prizes.build();
        self
    }

    pub fn draw(&mut self) -> PrizeItem {
        self.prizes.draw()
    }

    pub fn calculate_draw_rate(&self) -> Vec<(PrizeItem, f64)> {
        let mut draw_rate = Vec::new();
        // Calculate the total count of items
        let total_count = self.items.iter().map(|(_, count)| count).sum::<u64>() as f64;
        // Calculate the draw rate for each item
        // and store it in the HashMap
        for (item, count) in self.items.clone().into_iter() {
            let rate = ((count as f64) / total_count).max(0.0);
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
            (PrizeItem::new("Item1"), 2),
            (PrizeItem::new("Item2"), 3),
        ]);
        prizes.with_seed(12345);
        prizes.build();

        let drawn_item = prizes.draw();
        assert!(drawn_item.name == "Item1" || drawn_item.name == "Item2");
    }

    #[test]
    fn test_gashapon() {
        let mut gashapon = Gashapon::new();
        gashapon
            .add_item("Item1", 2)
            .add_item("Item2", 3)
            .with_seed(12345)
            .build();

        let drawn_item = gashapon.draw();
        assert!(drawn_item.name == "Item1" || drawn_item.name == "Item2");
    }

    #[test]
    fn test_gashapon_calculate_draw_rate() {
        let mut gashapon = Gashapon::new();
        gashapon.add_items(vec![("Item1", 2), ("Item2", 3), ("Item3", 5)]);
        gashapon.with_seed(12345).build();

        let draw_rate = gashapon.calculate_draw_rate();
        assert_eq!(draw_rate.len(), 3);
        assert!(
            draw_rate
                .iter()
                .map(|x| x.clone())
                .any(|(item, rate)| item.name == "Item1" && rate > 0.0)
        );
    }
}
