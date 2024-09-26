use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let length = prices.len();
    let mut dp: Vec<[i32; 2]> = vec![[0, 0]; length];
    dp[0][0] = 0;
    dp[0][1] = -prices[0];
    for i in 1..length {
        dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
        dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
    }

    return dp[length - 1][0];
}
