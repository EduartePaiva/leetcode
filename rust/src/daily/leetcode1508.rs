// 1508. Range Sum of Sorted Subarray Sums
pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, _n: i32, left: i32, right: i32) -> i32 {
        let mut new_array = vec![];

        for i in 0..nums.len() {
            let mut acc = 0;
            for j in i..nums.len() {
                acc += nums[j];
                new_array.push(acc);
            }
        }
        new_array.sort_unstable();
        const MODULE: i32 = 10_i32.pow(9) + 7;
        let mut res = 0;
        for i in (left - 1) as usize..right as usize {
            res += new_array[i];
            res %= MODULE;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    }
}
