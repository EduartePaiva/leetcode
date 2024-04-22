// 752. Open the Lock

use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    /// # Panics
    ///
    /// The function panics if `c` is not a digit from `0` to `9`.
    ///
    /// ```should_panic
    /// use lib::daily::leetcode0752::Solution;
    /// // panics here
    /// Solution::turning_logic('A');
    /// ```
    pub fn turning_logic(c: char) -> (char, char) {
        match c {
            '0' => ('1', '9'),
            '1' => ('0', '2'),
            '2' => ('1', '3'),
            '3' => ('2', '4'),
            '4' => ('3', '5'),
            '5' => ('4', '6'),
            '6' => ('5', '7'),
            '7' => ('6', '8'),
            '8' => ('7', '9'),
            '9' => ('8', '0'),
            _ => panic!("impossible case"),
        }
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        //vec of length 10000 vec 0 == 0000 vec 9999 = 9999, vec 1 == 0001,
        //so to wheel 1 is +1 wheel 01 is +10, 001 is +100, 0001 is +1000
        let visited: HashSet<(char, char, char, char)> =
            HashSet::from_iter(deadends.into_iter().map(|digits| {
                let mut iter = digits.chars();
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            }));

        // breath first search I want to get to 0000 from target

        let mut q: VecDeque<(char, char, char, char)> = VecDeque::new();
        q.push_back(target.chars().);

        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "reason"]
    fn test1() {
        assert_eq!(
            Solution::open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            ),
            6
        );
    }
}
