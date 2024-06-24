// 995. Minimum Number of K Consecutive Bit Flips
pub struct Solution;

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut window_flips = 0;
        let mut res = 0;

        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                window_flips -= 1;
            }
            if (window_flips + nums[i]) % 2 == 0 {
                if i + k > nums.len() {
                    return -1;
                }
                res += 1;
                window_flips += 1;
                nums[i] = 2;
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
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 1], 2), -1);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
