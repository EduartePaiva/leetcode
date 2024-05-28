// 1208. Get Equal Substrings Within Budget
pub struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, mut max_cost: i32) -> i32 {
        // pre compute the cost
        let cost: Vec<i32> = s
            .bytes()
            .into_iter()
            .zip(t.bytes().into_iter())
            .map(|(x, y)| x.abs_diff(y) as i32)
            .collect();
        // sliding window
        // while have max_cost grow to the right.
        // when max_cost isn't enough take one from left.
        // try to grow to the right again.

        let mut right = 0;
        let mut left = 0;
        let mut res = 0;

        while right < cost.len() {
            if max_cost >= cost[right] {
                max_cost -= cost[right];
                right += 1;
            } else if left < right {
                max_cost += cost[left];
                left += 1;
            } else {
                right += 1;
                left += 1;
            }
            res = res.max((right - left) as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 0),
            0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3),
            3
        );
    }
}
