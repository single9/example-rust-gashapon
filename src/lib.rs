use std::{
    collections::HashMap,
    time::{self, UNIX_EPOCH},
};

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
pub fn draw_prize_item(idx_box: &mut Vec<usize>, random_sort_items: &mut Vec<String>) -> String {
    // Seed the random number generator with the current time
    let mut seed = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
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
pub fn calculate_draw_rate(prize_items: &Vec<(&str, i32)>) -> HashMap<String, f32> {
    let mut draw_rate = HashMap::new();
    // Calculate the total count of items
    let total_count = prize_items.iter().map(|item| item.1).sum::<i32>() as f32;
    // Calculate the draw rate for each item
    // and store it in the HashMap
    for item in prize_items.iter() {
        let rate = (item.1 as f32) / total_count;
        draw_rate.insert(item.0.to_string(), rate);
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
        let drawn_item = draw_prize_item(&mut idx_box, &mut random_sort_items);
        assert_eq!(drawn_item, "A");
    }
}
