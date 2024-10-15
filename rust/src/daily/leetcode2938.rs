// 2938. Separate Black and White Balls
pub struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        // bring all zeros 0 to the destination.
        let mut res = 0;
        let mut dest = 0;
        for (i, n) in s.bytes().into_iter().enumerate() {
            if n == b'0' {
                res += (i - dest) as i64;
                dest += 1;
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
        assert_eq!(Solution::minimum_steps("101".to_string()), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_steps("100".to_string()), 2);
    }
}
