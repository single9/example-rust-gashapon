use std::time::{self, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub count: i64,
}

impl Item {
    pub fn new<T>(name: T, count: i64) -> Self
    where
        T: ToString,
    {
        Self {
            name: name.to_string(),
            count,
        }
    }
}

pub fn rng(seed: &mut usize) -> usize {
    *seed = (*seed).wrapping_mul(1103515245).wrapping_add(12345);
    (*seed >> 16) & 0x7FFF
}

pub fn random_sort<T: Clone>(items: &Vec<T>, seed: &mut usize) -> Vec<T> {
    let mut random_sorted_items = items.clone();
    for i in (0..items.len()).rev() {
        let j = (rng(seed) % (i + 1)) as usize;
        random_sorted_items.swap(i, j);
    }
    random_sorted_items
}

/// Draw a prize item from the given indices and mark it as "X" in the random sort items.
pub fn draw_prize_item(
    idx_box: &mut Vec<usize>,
    random_sort_items: &mut Vec<String>,
    seed: Option<usize>,
) -> String {
    // Seed the random number generator with the current time
    let mut seed = if let Some(seed) = seed {
        seed
    } else {
        time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    };
    // Generate a random index to select an item
    let r_idx = rng(&mut seed) % idx_box.len();
    // Get the index of the item to be drawn
    let idx = idx_box[r_idx];
    let my_prize = random_sort_items[idx].to_string();
    // Move the last element to the current index
    idx_box[r_idx] = idx_box[idx_box.len() - 1];
    // Remove the last element from the index box
    idx_box.pop();
    // Mark the selected item as "X" in the random sort items
    // This indicates that the item has been drawn
    // and is no longer available for drawing
    random_sort_items[idx] = "X".to_string();

    println!("Random index: {:?}, Val: {:?}", r_idx, idx);

    my_prize
}

/// Calculate the draw rate of each item based on the given prize items and their counts.
pub fn calculate_draw_rate(prize_items: &Vec<Item>) -> Vec<(&Item, f64)> {
    let mut draw_rate = Vec::new();
    // Calculate the total count of items
    let total_count = prize_items.iter().map(|item| item.count).sum::<i64>() as f64;
    // Calculate the draw rate for each item
    // and store it in the HashMap
    for item in prize_items.iter() {
        let rate = ((item.count as f64) / total_count).max(0.0);
        draw_rate.push((item, rate));
    }
    draw_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut seed = 12345;
        let random_number = rng(&mut seed);
        assert_eq!(random_number, 21468);
    }

    #[test]
    fn test_random_sort() {
        let items = vec![1, 2, 3, 4, 5];
        let mut seed = 12345;
        let sorted_items = random_sort(&items, &mut seed);
        assert_eq!(sorted_items.len(), items.len());
    }

    #[test]
    fn test_draw_prize_item() {
        let mut idx_box = vec![0, 1, 2];
        let mut random_sort_items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let drawn_item = draw_prize_item(&mut idx_box, &mut random_sort_items, Some(123455));
        assert_eq!(drawn_item, "B");
    }
}
