// 962. Maximum Width Ramp
pub struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut window = 0;
        let mut max_right = vec![0; nums.len()];
        max_right[nums.len() - 1] = nums[nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            max_right[i] = nums[i].max(max_right[i + 1]);
        }

        let mut left = 0;

        for right in 0..nums.len() {
            while nums[left] > max_right[right] {
                left += 1;
            }
            window = window.max(right - left);
        }

        window as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    }
}
