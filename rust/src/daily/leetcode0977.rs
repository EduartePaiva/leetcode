// 977. Squares of a Sorted Array
pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; nums.len()];
        let mut left = 0;
        let mut right = nums.len() - 1;

        for i in (0..nums.len()).rev() {
            let nl = nums[left].abs();
            let nr = nums[right].abs();
            if nl >= nr {
                res[i] = nl * nl;
                left += 1;
            } else {
                res[i] = nr * nr;
                right -= 1;
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
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
    }
}
