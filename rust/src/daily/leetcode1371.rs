// 1371. Find the Longest Substring Containing Vowels in Even Counts
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut hash: HashMap<u8, i32> = HashMap::new();
        let s = s.as_bytes();
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let mut prev = if vowels.contains(&s[0]) {
            hash.insert(s[0], 0);
            s[0]
        } else {
            0
        };
        hash.insert(0, -1);
        //amntyyaw
        let mut res = 0;
        for i in 1..s.len() {
            let current = if vowels.contains(&s[i]) { s[i] } else { 0 } ^ prev;
            if let Some(prev_i) = hash.get(&current) {
                res = res.max(i as i32 - prev_i);
            } else {
                hash.insert(current, i as i32);
            }
            prev = current;
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
            Solution::find_the_longest_substring("eleetminicoworoep".to_string()),
            13
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_the_longest_substring("leetcodeisgreat".to_string()),
            5
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_the_longest_substring("amntyyaw".to_string()),
            8
        );
    }
}
