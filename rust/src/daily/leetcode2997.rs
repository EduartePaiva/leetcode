// 2997. Minimum Number of Operations to Make Array XOR Equal to K
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        //boring answer
        return nums.into_iter().fold(k, |prev, n| prev ^ n).count_ones() as i32;

        /*
        cool answer

        let mut xor_all = 0;
        for num in &nums {
            xor_all ^= *num;
        }

        let mut count = 0;

        for i in 0..22 {
            if (xor_all & (1 << i)) != (k & (1 << i)) {
                count += 1;
            }
        }

        count

         */
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
    }
}
