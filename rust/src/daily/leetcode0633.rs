// 633. Sum of Square Numbers
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut squares = HashSet::new();
        let end = (c as f64).sqrt() as i32;
        for num in 0..=end {
            squares.insert(num * num);
        }
        for num in 0..=end {
            if squares.contains(&(c - (num * num))) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::judge_square_sum(5), true);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::judge_square_sum(4), true);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::judge_square_sum(2147482647), false);
    }
}
