// 648. Replace Words
pub struct Solution;

#[derive(Debug, Default)]
struct Trie {
    is_word: bool,
    next_characters: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut head = self;
        for &c in word.as_bytes() {
            let index = (c - b'a') as usize;
            head = head.next_characters[index].get_or_insert_with(Default::default);
        }
        head.is_word = true;
    }

    fn first_word_depth<'a>(&self, string_slice: &'a str) -> Option<&'a str> {
        let mut head = self;
        for (i, c) in string_slice.as_bytes().into_iter().enumerate() {
            let index = (c - b'a') as usize;
            if let Some(next_head) = head.next_characters[index].as_ref() {
                if next_head.is_word {
                    return Some(&string_slice[0..=i]);
                }
                head = &next_head;
            } else {
                break;
            }
        }
        None
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut root = Trie::new();
        for word in dictionary {
            root.insert(word);
        }
        let mut splitted_sentence: Vec<&str> = sentence.split_ascii_whitespace().collect();
        for i in 0..splitted_sentence.len() {
            if let Some(word) = root.first_word_depth(&splitted_sentence[i]) {
                splitted_sentence[i] = word;
            }
        }
        return splitted_sentence.join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat".to_string()
        )
    }
}
