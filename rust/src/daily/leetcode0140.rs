// 140. Word Break II
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        // I could use a trie, but first let's use a hashSet.
        let mut set: HashSet<&[u8]> = HashSet::new();
        for word in &word_dict {
            set.insert(word.as_bytes());
        }
        let s = s.as_bytes();
        fn dfs<'a>(
            set: &HashSet<&[u8]>,
            s: &'a [u8],
            acc: &mut Vec<&'a [u8]>,
            res: &mut Vec<String>,
            cur_i: usize,
        ) {
            if cur_i == s.len() {
                let mut new_string = String::new();
                for w in acc.iter() {
                    for &c in w.iter() {
                        new_string.push(c as char);
                    }
                    new_string.push(' ');
                }
                new_string.pop();
                res.push(new_string);
            }

            for i in cur_i..=s.len() {
                if set.contains(&s[cur_i..i]) {
                    acc.push(&s[cur_i..i]);
                    dfs(set, s, acc, res, i);
                    acc.pop();
                }
            }
        }
        let mut acc = vec![];
        let mut res: Vec<String> = vec![];
        dfs(&set, s, &mut acc, &mut res, 0);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::word_break(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "and".to_string(),
                    "sand".to_string(),
                    "dog".to_string()
                ]
            ),
            vec!["cat sand dog".to_string(), "cats and dog".to_string()]
        );
    }
}
