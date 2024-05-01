// 2000. Reverse Prefix of Word
pub struct Solution;

impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(index) = word.find(ch) {
            let end_word = word.split_off(index + 1);
            word = word.chars().rev().collect();
            word.push_str(&end_word);
        }
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_prefix(String::from("abcdefd"), 'd'),
            String::from("dcbaefd")
        );
    }
}
