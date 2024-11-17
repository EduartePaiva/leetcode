// 862. Shortest Subarray with Sum at Least K
pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut res = usize::MAX;
        let mut q: VecDeque<(i64, usize)> = VecDeque::new();
        let mut cur_sum = 0;
        for r in 0..nums.len() {
            cur_sum += nums[r] as i64;
            if cur_sum >= k {
                res = res.min(r + 1);
            }
            while !q.is_empty() && cur_sum - q.front().unwrap().0 >= k {
                res = res.min(r - q.pop_front().unwrap().1);
            }

            while !q.is_empty() && q.back().unwrap().0 > cur_sum {
                q.pop_back();
            }
            q.push_back((cur_sum, r));
        }

        if res == usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_subarray(vec![17, 85, 93, -45, -21], 150),
            2
        );
    }
}
