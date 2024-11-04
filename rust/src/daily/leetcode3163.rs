// 3163. String Compression III
pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut stack: Vec<(char, u8)> = vec![];
        for cur_c in word.chars() {
            if !stack.is_empty() {
                let lst_idx = stack.len() - 1;
                if stack[lst_idx].0 == cur_c && stack[lst_idx].1 < 9 {
                    stack[lst_idx].1 += 1;
                    continue;
                }
            }
            stack.push((cur_c, 1));
        }

        stack
            .into_iter()
            .map(|(c, qtd)| format!("{qtd}{c}"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e".to_string()
        );
    }
}
