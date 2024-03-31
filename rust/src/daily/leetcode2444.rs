// 2444. Count Subarrays With Fixed Bounds

pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let len = nums.len();
        let mut res = 0;
        let mut far_l = 0;
        let mut last_max_index = len;
        let mut last_min_index = len;

        for r in 0..len {
            if nums[r] == min_k {
                last_min_index = r;
            }
            if nums[r] == max_k {
                last_max_index = r;
            }
            if nums[r] < min_k || nums[r] > max_k {
                last_max_index = len;
                last_min_index = len;
                far_l = r + 1;
            }
            if last_max_index != len && last_min_index != len {
                let near_l = last_max_index.min(last_min_index);
                res += (near_l - far_l + 1) as i64;
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
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    689862, 297861, 946099, 25145, 946099, 647669, 863241, 886257, 946099, 25145,
                    567132, 484586, 478308, 427044, 545054, 25145, 25145, 25145, 25145, 25145
                ],
                25145,
                946099
            ),
            122
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    934372, 927845, 479424, 49441, 17167, 17167, 65553, 927845, 17167, 927845,
                    17167, 425106, 17167, 927845, 17167, 927845, 251338, 17167
                ],
                17167,
                927845
            ),
            118
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    35054, 398719, 945315, 945315, 820417, 945315, 35054, 945315, 171832, 945315,
                    35054, 109750, 790964, 441974, 552913
                ],
                35054,
                945315
            ),
            81
        );
    }
}
