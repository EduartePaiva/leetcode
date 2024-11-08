// 1829. Maximum XOR for Each Query
pub struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mask = 2_i32.pow(maximum_bit as u32) - 1;
        let mut prefix_xor = 0;
        for n in nums {
            prefix_xor ^= n;
            res.push((prefix_xor & mask) ^ mask);
        }
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
            vec![0, 3, 2, 3]
        );
    }
}
