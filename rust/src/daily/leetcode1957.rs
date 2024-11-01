// 1957. Delete Characters to Make Fancy String
pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut fancy_string: Vec<char> = Vec::new();
        for c in s.chars() {
            if fancy_string.len() < 2 {
                fancy_string.push(c);
                continue;
            }
            let lst_idx = fancy_string.len() - 1;
            if c == fancy_string[lst_idx] && c == fancy_string[lst_idx - 1] {
                continue;
            }
            fancy_string.push(c);
        }

        fancy_string.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
    }
}
