// 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence
pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split_whitespace()
            .enumerate()
            .find_map(|(idx, w)| {
                if w.starts_with(&search_word) {
                    Some((idx + 1) as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        )
    }
}
