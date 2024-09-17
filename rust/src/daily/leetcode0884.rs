// 884. Uncommon Words from Two Sentences
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words: HashMap<&str, i32> = HashMap::new();
        for w in s1.split_whitespace() {
            *words.entry(w).or_insert(0) += 1;
        }
        for w in s2.split_whitespace() {
            *words.entry(w).or_insert(0) += 1;
        }

        words
            .into_iter()
            .filter(|w| w.1 == 1)
            .map(|w| w.0.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            ),
            vec!["sour", "sweet"]
        );
    }
}
