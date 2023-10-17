pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::max_value();
    let mut max_profit = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
            println!("min price: {min_price}");
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
            println!("max prof: {max_profit}");
        }
    }

    max_profit
}