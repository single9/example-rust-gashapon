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

// fn main() {
//   // Define the prize items and their counts
//   let prize_items = vec![
//       ("S", 1),
//       ("A", 2),
//       ("B", 3),
//       ("C", 5),
//       ("D", 12),
//       ("E", 15),
//       ("F", 20),
//       ("G", 22),
//   ];
//   // Calculate the draw rate of each item
//   let draw_rate = calculate_draw_rate(&prize_items)
//       .into_iter()
//       .map(|(k, v)| (k, v * 100.0))
//       .collect::<HashMap<String, f32>>();
//   // Sort the draw rate by value
//   let mut draw_rate: Vec<_> = draw_rate.into_iter().collect();
//   draw_rate.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

//   // Count the total number of items
//   let prize_item_count = prize_items
//       .clone()
//       .into_iter()
//       .map(|item| item.1)
//       .sum::<i32>();
//   let prize_item_count = prize_item_count as usize;
//   // Create a vector of items based on the prize items and their counts
//   let items = {
//       let mut items = Vec::new();
//       for i in prize_items.into_iter() {
//           let item = i.0;
//           let mut item_count = i.1;
//           while item_count > 0 {
//               items.push(item.to_string());
//               item_count -= 1;
//           }
//       }
//       items
//   };
//   // Randomly sort the items
//   let mut random_sort_items = {
//       let mut seed: usize = 13245; // Use a fixed seed for reproducibility
//       random_sort(&items, &mut seed)
//   };

//   println!("Original items: {:?}", items);
//   println!("Randomly sorted items: {:?}", random_sort_items);

//   // Create a vector of indices for the items
//   let idx_box = {
//       let mut idx_box = Vec::new();
//       for i in 0..prize_item_count {
//           idx_box.push(i);
//       }
//       idx_box
//   };

//   // Randomly sort the indices of the items
//   let mut seed: usize = 13245;
//   let mut idx_box = random_sort(&idx_box, &mut seed);

//   println!("Randomly sorted indices: {:?}", idx_box);

//   loop {
//       // Draw a prize item from the random sort items
//       let my_prize = draw_prize_item(&mut idx_box, &mut random_sort_items);

//       println!("Remaining index: {:?}", idx_box);
//       println!("Remaining items: {:?}", random_sort_items);
//       println!("Draw item: {:?}", my_prize);

//       if idx_box.len() == 0 {
//           println!("No more items left to draw.");
//           break;
//       }
//   }

//   println!("Draw rates (%): {:?}", draw_rate);
// }
