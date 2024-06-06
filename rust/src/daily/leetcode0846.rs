// 846. Hand of Straights
pub struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }
        let mut count = HashMap::new();
        for num in hand {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut sorted_count: Vec<[i32; 2]> =
            count.into_iter().map(|(key, cnt)| [key, cnt]).collect();
        sorted_count.sort_unstable();
        let mut sorted_count = VecDeque::from(sorted_count);

        while !sorted_count.is_empty() {
            if sorted_count.len() < group_size || sorted_count[0][1] < 0 {
                return false;
            }
            let num_ref = sorted_count[0][0];
            for i in 0..group_size {
                if i as i32 + num_ref != sorted_count[i][0] {
                    return false;
                }
                sorted_count[i][1] -= 1;
            }
            while !sorted_count.is_empty() && sorted_count[0][1] == 0 {
                sorted_count.pop_front();
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
            true
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::is_n_straight_hand(vec![8, 10, 12], 3), false);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6], 2),
            true
        );
    }
}
