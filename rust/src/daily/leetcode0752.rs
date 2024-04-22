// 752. Open the Lock

pub struct Solution;

use std::collections::{HashSet, VecDeque};
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
    pub fn turning_logic(digit: char) -> (char, char) {
        match digit {
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
        let mut visited: HashSet<[char; 4]> =
            HashSet::from_iter(deadends.into_iter().map(|digits| {
                let mut iter = digits.chars();
                [
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                ]
            }));

        // breath first search I want to get to 0000 from target

        let mut q: VecDeque<[char; 4]> = VecDeque::new();

        let mut target = target.chars();
        let target = [
            target.next().unwrap(),
            target.next().unwrap(),
            target.next().unwrap(),
            target.next().unwrap(),
        ];
        q.push_back(target);

        let mut steps = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(mut cur_digits) = q.pop_front() {
                    if cur_digits == ['0', '0', '0', '0'] {
                        return steps;
                    }
                    for i in 0..4 {
                        let digit = cur_digits[i];
                        let possible_digits = Solution::turning_logic(digit);
                        cur_digits[i] = possible_digits.0;

                        if !visited.contains(&cur_digits) {
                            q.push_back(cur_digits);
                            visited.insert(cur_digits);
                        }
                        cur_digits[i] = possible_digits.1;
                        if !visited.contains(&cur_digits) {
                            q.push_back(cur_digits);
                            visited.insert(cur_digits);
                        }
                        cur_digits[i] = digit;
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
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
    #[test]
    fn test2() {
        assert_eq!(
            Solution::open_lock(vec!["8888".to_string(),], "0009".to_string()),
            1
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::open_lock(vec!["0000".to_string(),], "8888".to_string()),
            -1
        );
    }
}
