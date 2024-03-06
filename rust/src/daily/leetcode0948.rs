//948. Bag of Tokens
pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort_unstable();
        let mut tokens: VecDeque<i32> = tokens.into_iter().collect();
        let mut score = 0;

        loop {
            if !tokens.is_empty() && tokens[0] <= power {
                score += 1;
                power -= tokens.pop_front().unwrap();
                continue;
            }

            if tokens.len() > 1 && score > 0 {
                power += tokens.pop_back().unwrap();
                score -= 1;
                continue;
            }
            break score;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 100], 150), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::bag_of_tokens_score(vec![71, 55, 82], 54), 0);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
    }
}
