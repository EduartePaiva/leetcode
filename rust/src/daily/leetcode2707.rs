// 2707. Extra Characters in a String
pub struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        for i in (0..s.len()).rev() {
            let mut min_skip = dp[i + 1] + 1;
            for w in &dictionary {
                if s[i..].starts_with(w.as_bytes()) {
                    min_skip = min_skip.min(dp[i + w.len()]);
                }
            }
            dp[i] = min_skip;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                vec![
                    "leet".to_string(),
                    "code".to_string(),
                    "leetcode".to_string()
                ]
            ),
            1
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_extra_char(
                "fvobfydblzrqrohprlhayxqqjkhi".to_string(),
                vec![
                    "uhuy".to_string(),
                    "q".to_string(),
                    "oy".to_string(),
                    "joi".to_string(),
                    "sqvng".to_string(),
                    "l".to_string(),
                    "ha".to_string(),
                    "vobfy".to_string(),
                    "rnhs".to_string(),
                    "d".to_string(),
                    "o".to_string(),
                    "eyv".to_string(),
                    "nl".to_string(),
                    "xpyb".to_string(),
                    "acpl".to_string(),
                    "br".to_string(),
                    "c".to_string(),
                    "xpcvu".to_string(),
                    "b".to_string(),
                    "k".to_string(),
                    "z".to_string(),
                    "eybel".to_string(),
                    "jzl".to_string(),
                    "y".to_string(),
                    "beu".to_string(),
                    "ps".to_string(),
                    "rnvc".to_string(),
                    "lzrq".to_string(),
                    "v".to_string(),
                    "ki".to_string(),
                    "qrqof".to_string(),
                    "r".to_string(),
                    "gh".to_string(),
                    "qszzj".to_string(),
                    "cug".to_string(),
                    "wccu".to_string(),
                    "xqqj".to_string(),
                    "jzgi".to_string(),
                    "a".to_string(),
                    "os".to_string(),
                    "ytk".to_string(),
                    "s".to_string(),
                    "ohprl".to_string()
                ]
            ),
            3
        )
    }
}
