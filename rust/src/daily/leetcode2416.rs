// 2416. Sum of Prefix Scores of Strings
pub struct Solution;

const ARRAY_REPEAT_VALUE: Option<Box<Trie>> = None;
struct Trie {
    num_words: i32,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self {
            num_words: 0,
            children: [ARRAY_REPEAT_VALUE; 26],
        }
    }

    fn insert(&mut self, word: &[u8]) {
        let mut head = self;
        for w in word {
            let index = (w - b'a') as usize;
            if head.children[index].is_none() {
                head.children[index] = Some(Box::new(Trie::new()));
            }
            head.num_words += 1;
            head = head.children[index].as_mut().unwrap();
        }
        head.num_words += 1;
    }

    fn get_score(&self, word: String) -> i32 {
        let mut head = self;
        let mut res = 0;
        for c in word.bytes() {
            let i = (c - b'a') as usize;
            head = head.children[i].as_ref().unwrap();
            res += head.num_words;
        }

        res
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];
        let mut trie = Trie::new();
        for w in &words {
            trie.insert(w.as_bytes());
        }
        for word in words {
            res.push(trie.get_score(word));
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
            Solution::sum_prefix_scores(vec![
                "abc".to_string(),
                "ab".to_string(),
                "bc".to_string(),
                "b".to_string()
            ]),
            vec![5, 4, 3, 2]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sum_prefix_scores(vec![
                "qtcqcmwcin".to_string(),
                "vkjotbrbzn".to_string(),
                "eoorlyfche".to_string(),
                "eoorlyhn".to_string(),
                "eoorlyfcxk".to_string(),
                "qfnmjilcom".to_string(),
                "eoorlyfche".to_string(),
                "qtcqcmwcnl".to_string(),
                "qtcqcrpjr".to_string()
            ]),
            [24, 10, 34, 26, 32, 13, 34, 24, 20]
        )
    }
}
