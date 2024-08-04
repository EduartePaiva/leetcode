// 1508. Range Sum of Sorted Subarray Sums
pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut new_array = vec![];

        for i in 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    }
}
