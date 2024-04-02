// 205. Isomorphic Strings
pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut added = HashSet::new();
        let s = s.as_bytes();
        let t = t.as_bytes();

        //"badc", "baba"
        for i in 0..s.len() {
            match map.get(&s[i]) {
                Some(val) => {
                    if *val != t[i] {
                        return false;
                    }
                }
                None => {
                    if added.contains(&t[i]) {
                        return false;
                    }
                    map.insert(s[i], t[i]);
                    added.insert(t[i]);
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
            false
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }
}
