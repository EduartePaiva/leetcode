// 2554. Maximum Number of Integers to Choose From a Range I
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned: HashSet<i32> = HashSet::from_iter(banned.into_iter());
        let mut res = 0;
        let mut sum = 0;
        for num in 1..=n {
            if banned.contains(&num) {
                continue;
            }
            if sum + num > max_sum {
                break;
            }
            res += 1;
            sum += num;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2)
    }
}
