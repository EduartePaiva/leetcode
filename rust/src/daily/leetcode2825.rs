// 2825. Make String a Subsequence Using Cyclic Increments
pub struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        if str2.len() > str1.len() {
            return false;
        }
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();

        let mut s1_i = 0;
        let mut s2_i = 0;

        while s1_i < str1.len() && s2_i < str2.len() {
            if str1[s1_i] == str2[s2_i] || (((str1[s1_i] + 1) - b'a') % 26) == (str2[s2_i] - b'a') {
                s2_i += 1;
            }

            s1_i += 1;
        }

        s2_i == str2.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_make_subsequence("abc".to_string(), "ad".to_string()),
            true
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_make_subsequence("zc".to_string(), "ad".to_string()),
            true
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::can_make_subsequence("ab".to_string(), "d".to_string()),
            false
        )
    }
}
