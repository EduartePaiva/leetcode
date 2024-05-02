// 2441. Largest Positive Integer That Exists With Its Negative
pub struct Solution;

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        println!("{:?}", nums);

        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right && nums[left] < nums[right] {
            let max_pos = nums[right];
            while left < right && nums[left].abs() > max_pos {
                left += 1;
            }
            if nums[left] > 0 {
                break;
            }
            if nums[left].abs() == nums[right] {
                return nums[right];
            }
            right -= 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_max_k(vec![-24, 8, 6, 46, -45, -5, 37, -10, 1]),
            -1
        );
    }
}
