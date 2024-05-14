pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        // 设定一个最小值，这样可以动态与最小的比较以及动态设定最大收益
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices.iter().skip(1) {
            if *price < min_price {
                min_price = *price;
            } else {
                let profit = *price - min_price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }

        max_profit
    }
}
