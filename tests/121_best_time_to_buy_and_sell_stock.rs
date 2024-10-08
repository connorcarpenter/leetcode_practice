// 121. Best Time to Buy and Sell Stock
// From: https://leetcode.com/problems/best-time-to-buy-and-sell-stock
//
// You are given an array prices where prices[i] is the price of a given stock on the ith day.
//
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

// Example 1:
//
// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
//
// Example 2:
//
// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.
//
// Constraints:
//
// 1 <= prices.length <= 105
// 0 <= prices[i] <= 104
//
// Solved in 28 minutes, 37 seconds

#[test]
fn official_example_1() {
    let input = vec![7,1,5,3,6,4];
    let output = Solution::max_profit(input);
    assert_eq!(output, 5);
}

#[test]
fn official_example_2() {
    let input = vec![7,6,4,3,1];
    let output = Solution::max_profit(input);
    assert_eq!(output, 0);
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let prices_len = prices.len();

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for i in 1..prices_len {
            let profit = prices[i] - min_price;
            if profit > max_profit {
                max_profit = profit;
            }

            if prices[i] < min_price {
                min_price = prices[i];
            }
        }

        max_profit
    }
}