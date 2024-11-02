// 2490. Circular Sentence
pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.as_bytes();
        if sentence[0] != sentence[sentence.len() - 1] {
            return false;
        }
        for i in 0..sentence.len() {
            if sentence[i] == b' ' {
                if sentence[i - 1] != sentence[i + 1] {
                    return false;
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
            Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()),
            true
        );
    }
}
