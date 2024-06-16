// 330. Patching Array
pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        // 1,3 target = 6
        //

        let mut miss = 1_i64;
        let mut result = 0;
        let mut i = 0;
        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 1);
    }
}
