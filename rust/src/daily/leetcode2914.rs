// 2914. Minimum Number of Changes to Make Binary String Beautiful
pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut res = 0;
        let s = s.as_bytes();
        for window in s.chunks(2) {
            if window[0] != window[1] {
                res += 1;
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
        assert_eq!(Solution::min_changes("1001".to_string()), 2);
    }
}
