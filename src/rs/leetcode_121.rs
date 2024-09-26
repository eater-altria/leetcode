pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    let length = prices.len();

    for i in 1..length {
        let profit = prices[i] - min_price;
        if profit > max_profit {
            max_profit = profit;
        }
        if prices[i] < min_price {
            min_price = prices[i]
        }
    }
    max_profit
}
