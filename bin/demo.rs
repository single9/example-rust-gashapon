use test_gashapon::{Item, calculate_draw_rate, draw_prize_item, random_sort};

fn main() {
    // Define the prize items and their counts
    let prize_items = vec![
        Item::new("S", 1),
        Item::new("A", 2),
        Item::new("B", 3),
        Item::new("C", 5),
        Item::new("D", 12),
        Item::new("E", 15),
        Item::new("F", 20),
        Item::new("G", 22),
    ];
    // Calculate the draw rate of each item
    let draw_rate = calculate_draw_rate(&prize_items)
        .into_iter()
        .map(|(k, v)| (k, v * 100.0))
        .collect::<Vec<_>>();
    // Sort the draw rate by value
    let mut draw_rate: Vec<_> = draw_rate.into_iter().collect();
    draw_rate.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Count the total number of items
    let prize_item_count = prize_items
        .clone()
        .into_iter()
        .map(|item| item.count)
        .sum::<i64>();
    let prize_item_count = prize_item_count as usize;
    // Create a vector of items based on the prize items and their counts
    let items = {
        let mut items = Vec::new();
        for i in prize_items.clone().into_iter() {
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

    println!(
        "Draw rates (%): {:?}",
        draw_rate
            .iter()
            .map(|(k, v)| (k.name.clone(), *v))
            .collect::<Vec<_>>()
    );
}
