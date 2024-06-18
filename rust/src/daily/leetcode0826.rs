//826. Most Profit Assigning Work

pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        // zip difficult and profit
        let mut difficult_w_profit = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<(i32, i32)>>();
        difficult_w_profit.sort_unstable();
        difficult_w_profit.reverse();
        worker.sort_unstable();

        let mut most_profit = BinaryHeap::new();
        let mut total_profit = 0;
        for w in worker {
            while !difficult_w_profit.is_empty() && difficult_w_profit.last().unwrap().0 <= w {
                let (_, profit) = difficult_w_profit.pop().unwrap();
                most_profit.push(profit);
            }
            total_profit += most_profit.peek().unwrap_or(&0);
        }

        total_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }
}
