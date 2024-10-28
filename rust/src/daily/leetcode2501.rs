// 2501. Longest Square Streak in an Array
pub struct Solution;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut max_streak = -1;
        let max_val = nums[nums.len() - 1];
        for &n in &nums {
            // calculate the streak of n
            let mut streak = 1;
            let mut next = n * n;
            if next > max_val {
                break;
            }
            while next <= max_val && nums.binary_search(&next).is_ok() {
                streak += 1;
                next *= next;
            }
            if streak > 1 {
                max_streak = max_streak.max(streak);
            }
        }
        max_streak
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
    }
}
