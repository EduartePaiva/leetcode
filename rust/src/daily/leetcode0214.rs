// 214. Shortest Palindrome
pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let s = s
            .as_bytes()
            .iter()
            .map(|c| c + 1 - b'a')
            .collect::<Vec<u8>>();

        let mut ls = 0;
        let mut straight_hash = 0;
        let mut reverse_hash = 0;
        for i in 0..s.len() {
            straight_hash += s[i] as i128 * 29_i128.pow(i as u32);
            reverse_hash = s[i] as i128 + reverse_hash * 29;
            if straight_hash == reverse_hash {
                ls = i;
            }
        }
        let suffix = s[ls + 1..s.len()]
            .iter()
            .rev()
            .map(|c| (c - 1 + b'a') as char)
            .collect::<String>();
        let second = s.iter().map(|c| (c - 1 + b'a') as char).collect::<String>();
        return String::from_iter([suffix, second]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa".to_string()
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd".to_string()
        );
    }
}
