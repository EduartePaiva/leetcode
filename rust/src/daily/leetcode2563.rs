// 2563. Count the Number of Fair Pairs
pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut res = 0;
        nums.sort_unstable();
        fn binary_search(nums: &Vec<i32>, mut l: usize, mut r: usize, target: i32) -> usize {
            while l <= r {
                let m = ((r - l) / 2) + l;
                if nums[m] >= target {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }
            r
        }
        for i in 0..nums.len() {
            let low = lower - nums[i];
            let up = upper - nums[i];
            res += (binary_search(&nums, i + 1, nums.len() - 1, up + 1)
                - binary_search(&nums, i + 1, nums.len() - 1, low)) as i64;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
