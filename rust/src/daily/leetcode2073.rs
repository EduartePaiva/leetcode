// 2073. Time Needed to Buy Tickets
pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut tickets_heap = BinaryHeap::with_capacity(tickets.len());
        for (i, val) in tickets.into_iter().enumerate() {
            tickets_heap.push((Reverse(val), i))
        }

        let mut cur_time = 0;
        let mut prev_val = 0;
        let mut i_shift = 0;

        while !tickets_heap.is_empty() {
            let start_heap_length = tickets_heap.len() as i32;
            let top_val = tickets_heap.peek().unwrap().0 .0;
            cur_time += (top_val - prev_val) * start_heap_length;
            prev_val = top_val;
            while !tickets_heap.is_empty() && tickets_heap.peek().unwrap().0 .0 == top_val {
                if let Some((_, i)) = tickets_heap.pop() {
                    if i < k {
                        i_shift += 1;
                    } else if i == k {
                        return cur_time - (start_heap_length - 1 - (i - i_shift) as i32);
                    }
                }
            }
        }
        cur_time
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::time_required_to_buy(vec![84, 49, 5, 24, 70, 77, 87, 8], 3),
            154
        );
    }
}
