// 2053. Kth Distinct String in an Array
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut map = HashMap::new();
        for str in &arr {
            *map.entry(str).or_insert(0) += 1;
        }
        for str in &arr {
            if let Some(&val) = map.get(str) {
                if val == 1 {
                    k -= 1;
                    if k == 0 {
                        return String::from(str);
                    }
                } else {
                    map.remove(str);
                }
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a".to_string()
        )
    }
}
