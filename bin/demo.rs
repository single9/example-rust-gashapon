use test_gashapon;

fn main() {
    let mut gashpon = test_gashapon::Gashapon::new();
    gashpon.add_items(vec![
        ("S", 1),
        ("A", 2),
        ("B", 3),
        ("C", 5),
        ("D", 12),
        ("E", 15),
        ("F", 20),
        ("G", 22),
    ]);
    gashpon.with_seed(12345).build();
    // Calculate the draw rate of each item
    let draw_rate = gashpon
        .calculate_draw_rate()
        .into_iter()
        .map(|(k, v)| (k, v * 100.0))
        .collect::<Vec<_>>();
    // Sort the draw rate by value
    let mut draw_rate: Vec<_> = draw_rate.into_iter().collect();
    draw_rate.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("Original items: {:?}", gashpon.items);
    println!(
        "Randomly sorted items: {:?}",
        gashpon.prizes.get_randomized_items()
    );

    println!("Randomly sorted indices: {:?}", gashpon.prizes.idx_box);

    loop {
        // Draw a prize item from the random sort items
        let my_prize = gashpon.draw();

        println!("Remaining index: {:?}", gashpon.prizes.idx_box);
        println!(
            "Remaining items: {:?}",
            gashpon.prizes.get_randomized_items()
        );
        println!("Draw item: {:?}", my_prize);

        if gashpon.prizes.idx_box.len() == 0 {
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
