// 502. IPO
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // tuple of capital and profit
        let mut capital_w_profit = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<(i32, i32)>>();

        capital_w_profit.sort_unstable();
        capital_w_profit.reverse();
        let mut cur_profits = BinaryHeap::new();
        for _ in 0..k {
            while !capital_w_profit.is_empty() && capital_w_profit.last().unwrap().0 <= w {
                let (_, profit) = capital_w_profit.pop().unwrap();
                cur_profits.push(profit);
            }
            if let Some(capital) = cur_profits.pop() {
                w += capital;
            } else {
                break;
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
    }
}
