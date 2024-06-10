// 1051. Height Checker
pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted_heights = heights.clone();
        sorted_heights.sort_unstable();
        let mut res = 0;
        for i in 0..heights.len() {
            if heights[i] != sorted_heights[i] {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }
}
