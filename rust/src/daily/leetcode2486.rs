// 2486. Append Characters to String to Make Subsequence
pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut res = t.len() as i32;
        let mut s_i = 0;
        let mut t_i = 0;

        while s_i < s.len() && t_i < t.len() {
            if s[s_i] == t[t_i] {
                t_i += 1;
                res -= 1;
            }
            s_i += 1;
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
            Solution::append_characters("coaching".to_string(), "coding".to_string()),
            4
        );
    }
}
