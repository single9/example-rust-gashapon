use std::collections::HashMap;

use test_gashapon::{calculate_draw_rate, draw_prize_item, random_sort};

fn main() {
    // Define the prize items and their counts
    let prize_items = vec![
        ("S", 1),
        ("A", 2),
        ("B", 3),
        ("C", 5),
        ("D", 12),
        ("E", 15),
        ("F", 20),
        ("G", 22),
    ];
    // Calculate the draw rate of each item
    let draw_rate = calculate_draw_rate(&prize_items)
        .into_iter()
        .map(|(k, v)| (k, v * 100.0))
        .collect::<HashMap<String, f32>>();
    // Sort the draw rate by value
    let mut draw_rate: Vec<_> = draw_rate.into_iter().collect();
    draw_rate.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Count the total number of items
    let prize_item_count = prize_items
        .clone()
        .into_iter()
        .map(|item| item.1)
        .sum::<i32>();
    let prize_item_count = prize_item_count as usize;
    // Create a vector of items based on the prize items and their counts
    let items = {
        let mut items = Vec::new();
        for i in prize_items.into_iter() {
            let item = i.0;
            let mut item_count = i.1;
            while item_count > 0 {
                items.push(item.to_string());
                item_count -= 1;
            }
        }
        items
    };
    // Randomly sort the items
    let mut random_sort_items = {
        let mut seed: usize = 13245; // Use a fixed seed for reproducibility
        random_sort(&items, &mut seed)
    };

    println!("Original items: {:?}", items);
    println!("Randomly sorted items: {:?}", random_sort_items);

    // Create a vector of indices for the items
    let idx_box = {
        let mut idx_box = Vec::new();
        for i in 0..prize_item_count {
            idx_box.push(i);
        }
        idx_box
    };

    // Randomly sort the indices of the items
    let mut seed: usize = 13245;
    let mut idx_box = random_sort(&idx_box, &mut seed);

    println!("Randomly sorted indices: {:?}", idx_box);

    loop {
        // Draw a prize item from the random sort items
        let my_prize = draw_prize_item(&mut idx_box, &mut random_sort_items, None);

        println!("Remaining index: {:?}", idx_box);
        println!("Remaining items: {:?}", random_sort_items);
        println!("Draw item: {:?}", my_prize);

        if idx_box.len() == 0 {
            println!("No more items left to draw.");
            break;
        }
    }

    println!("Draw rates (%): {:?}", draw_rate);
}
