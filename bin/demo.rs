use test_gashapon::{Gashapon, GashaponItem, PrizeItem};

fn main() {
    let mut gashpon = Gashapon::new();
    gashpon.add_items(vec![
        GashaponItem::new(PrizeItem::new("S")).with_quantity(1),
        GashaponItem::new(PrizeItem::new("A")).with_quantity(2),
        GashaponItem::new(PrizeItem::new("B")).with_quantity(3),
        GashaponItem::new(PrizeItem::new("C")).with_quantity(5),
        GashaponItem::new(PrizeItem::new("D")).with_quantity(12),
        GashaponItem::new(PrizeItem::new("E")).with_quantity(15),
        GashaponItem::new(PrizeItem::new("F")).with_quantity(20),
        GashaponItem::new(PrizeItem::new("G")).with_quantity(22),
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
        println!("Items: {:?}", gashpon.items);
        if gashpon.prizes.idx_box.len() == 0 {
            println!("No more items left to draw.");
            break;
        }
    }

    println!(
        "Draw rates (%): {:?}",
        draw_rate
            .iter()
            .map(|(k, v)| (k.prize.name.clone(), *v))
            .collect::<Vec<_>>()
    );

    // Restore the items to the original state
    gashpon.restore_items();
    println!("Restored items: {:?}", gashpon.items);
}
