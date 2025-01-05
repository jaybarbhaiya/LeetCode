fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for (i, price) in prices.iter().enumerate() {
        if i > 0 && price > &prices[i - 1] {
            profit += price - prices[i - 1];
        }
    }
    profit
}

pub fn run() {
    let prices = vec![7, 6, 4, 3, 1];
    let result = max_profit(prices);
    println!("Result: {}", result);
}
