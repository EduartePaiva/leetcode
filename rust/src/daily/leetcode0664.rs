// 664. Strange Printer
pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }
        for l in 2..n + 1 {
            for i in 0..n - l + 1 {
                let j = i + l - 1;
                // print ith letter separately
                dp[i][j] = dp[i + 1][j] + 1;

                for k in i + 1..j + 1 {
                    if s[i] == s[k] {
                        dp[i][j] = dp[i][j].min(dp[i][k - 1] + if j > k { dp[k + 1][j] } else { 0 })
                    }
                }
            }
        }
        return dp[0][n - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2)
    }
}
