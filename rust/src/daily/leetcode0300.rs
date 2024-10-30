// 300. Longest Increasing Subsequence
pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lts = vec![nums[0]];
        for n in nums {
            if *lts.last().unwrap() < n {
                lts.push(n);
                continue;
            }
            if let Err(idx) = lts.binary_search(&n) {
                lts[idx] = n;
            }
        }
        lts.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
}
