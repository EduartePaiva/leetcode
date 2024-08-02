// 2134. Minimum Swaps to Group All 1's Together II
pub struct Solution;

impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let nums_of_1 = nums.iter().filter(|x| **x == 1).count() as i32;

        let mut window_1 = 0;
        for i in 0..nums_of_1 as usize {
            if nums[i] == 1 {
                window_1 += 1;
            }
        }
        let mut res = nums_of_1 - window_1;
        nums.extend(nums.clone());
        for i in nums_of_1 as usize..nums.len() {
            if nums[i - nums_of_1 as usize] == 1 {
                window_1 -= 1;
            }
            if nums[i] == 1 {
                window_1 += 1;
            }
            res = res.min(nums_of_1 - window_1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    }
}
