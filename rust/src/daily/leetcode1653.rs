// 1653. Minimum Deletions to Make String Balanced
pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count_a = s
            .iter()
            .fold(0, |prev, cur| if *cur == b'a' { prev + 1 } else { prev });
        // delete current b vs deleting all the next a if the cost is equal then delete a and return
        let mut count_b = 0;
        let mut res = s.len() as i32;
        for &c in s {
            if c == b'a' {
                count_a -= 1;
            }
            res = res.min(count_a + count_b);
            if c == b'b' {
                count_b += 1;
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
        assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_deletions("a".to_string()), 0);
    }
}
