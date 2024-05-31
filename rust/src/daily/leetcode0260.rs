// 260. Single Number III
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut accumulated_xor = 0;
        for &n in &nums {
            accumulated_xor ^= n;
        }
        let mut group1 = 0;
        let mut group2 = 0;
        let mut diff_bit = 1;
        while (diff_bit & accumulated_xor) == 0 {
            diff_bit <<= 1;
        }
        for &n in &nums {
            if (n & diff_bit) == 0 {
                group1 ^= n;
            } else {
                group2 ^= n;
            }
        }
        vec![group1, group2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![5, 3]);
    }
}
