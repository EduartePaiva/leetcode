// 2962. Count Subarrays Where Max Element Appears at Least K Times

pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_num = *nums.iter().max().unwrap();
        let mut l = 0;
        let mut res: i64 = 0;
        let mut r_count = 0;
        let mut l_count = 0;
        for r in 0..nums.len() {
            r_count += if nums[r] == max_num { 1 } else { 0 };
            while r_count - l_count >= k {
                l_count += if nums[l] == max_num { 1 } else { 0 };
                l += 1;
            }
            res += l as i64;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    81, 23, 38, 23, 56, 40, 82, 56, 82, 82, 82, 70, 8, 69, 8, 7, 19, 14, 58, 42,
                    82, 10, 82, 78, 15, 82
                ],
                2
            ),
            224
        );
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    28, 5, 58, 91, 24, 91, 53, 9, 48, 85, 16, 70, 91, 91, 47, 91, 61, 4, 54, 61, 49
                ],
                1
            ),
            187
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    37, 20, 38, 66, 34, 38, 9, 41, 1, 14, 25, 63, 8, 12, 66, 66, 60, 12, 35, 27,
                    16, 38, 12, 66, 38, 36, 59, 54, 66, 54, 66, 48, 59, 66, 34, 11, 50, 66, 42, 51,
                    53, 66, 31, 24, 66, 44, 66, 1, 66, 66, 29, 54
                ],
                5
            ),
            594
        );
    }
}
