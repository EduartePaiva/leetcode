// 950. Reveal Cards In Increasing Order

pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut res = VecDeque::with_capacity(deck.len());
        for cur_val in deck.into_iter().rev() {
            //put the bottom card at the top.
            if !res.is_empty() {
                res.rotate_right(1);
            }
            //take the current card and push it to the top of the deck
            res.push_front(cur_val);
        }

        Vec::from(res)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
    }
}
