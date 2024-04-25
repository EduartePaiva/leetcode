// 2370. Longest Ideal Subsequence
pub struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut memo = [0_u32; 26];

        for i in (0..s.len()).rev() {
            let cur_c = (s[i] - b'a') as usize;
            let min_c = if cur_c as i32 - k >= 0 {
                cur_c - k as usize
            } else {
                0
            };
            let max_c = if cur_c as i32 + k <= 25 {
                cur_c + k as usize
            } else {
                25
            };
            let mut max_sequence: u32 = 0;
            for i in min_c..=max_c {
                max_sequence = max_sequence.max(memo[i]);
            }
            memo[cur_c] = max_sequence + 1;
        }
        memo.into_iter().max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::longest_ideal_string("pvjcci".to_string(), 4), 2);
    }
}
