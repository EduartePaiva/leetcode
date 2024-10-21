// 1593. Split a String Into the Max Number of Unique Substrings
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut splits: HashSet<&[u8]> = HashSet::new();
        let s = s.as_bytes();
        // try every possible combination
        fn backtrack<'a>(
            left: usize,
            right: usize,
            s: &'a [u8],
            splits: &mut HashSet<&'a [u8]>,
        ) -> usize {
            if right == s.len() {
                return splits.len();
            }
            let mut res = 0;
            // include current
            if !splits.contains(&s[left..=right]) {
                splits.insert(&s[left..=right]);
                res = res.max(backtrack(right + 1, right + 1, s, splits));
                splits.remove(&s[left..=right]);
            }
            // skip current
            res = res.max(backtrack(left, right + 1, s, splits));
            res
        }
        backtrack(0, 0, s, &mut splits) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_unique_split("hq".to_string()), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_unique_split("wwwzfvedwfvhsww".to_string()),
            11
        );
    }
}
