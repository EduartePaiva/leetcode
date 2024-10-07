//1813. Sentence Similarity III
pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1.len() == sentence2.len() {
            if sentence1 == sentence2 {
                return true;
            } else {
                return false;
            }
        }
        let sent1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let sent2 = sentence2.split_whitespace().collect::<Vec<_>>();
        let (mut st1_l, mut st2_l, mut st1_r, mut st2_r) = (
            0_i32,
            0_i32,
            (sent1.len() - 1) as i32,
            (sent2.len() - 1) as i32,
        );
        while st1_l < sent1.len() as i32
            && st2_l < sent2.len() as i32
            && sent1[st1_l as usize] == sent2[st2_l as usize]
        {
            st1_l += 1;
            st2_l += 1;
        }
        if st1_l as usize == sent1.len() || st2_l as usize == sent2.len() {
            return true;
        }

        while st1_r >= 0 && st2_r >= 0 && sent1[st1_r as usize] == sent2[st2_r as usize] {
            st1_r -= 1;
            st2_r -= 1;
        }
        st1_l - 1 >= st1_r || st2_l - 1 >= st2_r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::are_sentences_similar(
                "Hello Jane".to_string(),
                "Hello my name is Jane".to_string()
            ),
            true
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string()),
            true
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::are_sentences_similar("a".to_string(), "a aa a".to_string()),
            true
        )
    }
}
