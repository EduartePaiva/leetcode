// 41. First Missing Positive
pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        //result can be between 1..=n+1
        let n = nums.len() as i32;

        //remove negatives and out of range numbers.
        for i in 0..nums.len() {
            if nums[i] > n || nums[i].is_negative() {
                nums[i] = 0;
            }
        }

        // swap numbers and swap signals for numbers in the right place
        // and remove duplicates by putting a 0 in their place
        for i in 0..nums.len() {
            while nums[i] > 0 {
                let next_i = nums[i] as usize - 1;
                let cur_num = -nums[i];

                // if cur_num is equal to nums[next_i] that means
                // nums[i] is a duplicate, so return 0
                // when in the next line I do nums[i] == next_num
                // if it's a duplicate nums[i] will be 0
                let next_num = if nums[next_i] == cur_num {
                    0
                } else {
                    nums[next_i]
                };

                nums[i] = next_num;
                nums[next_i] = cur_num;
            }
        }

        // return the position + 1 of the first 0 in nums,
        // if it don't find any 0 it return n + 1
        match nums.into_iter().position(|val| val == 0) {
            Some(index) => index as i32 + 1,
            None => n + 1,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::first_missing_positive(vec![2]), 1);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    }
    #[test]
    fn test5() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 6, 3, 5, 4]), 7);
    }
}
