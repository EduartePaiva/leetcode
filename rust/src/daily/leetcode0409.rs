// 409. Longest Palindrome
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = vec![0; 58];
        let mut res = 0;
        for &c in s.as_bytes() {
            let index = (c - b'A') as usize;
            count[index] += 1;
            if count[index] == 2 {
                count[index] -= 2;
                res += 2;
            }
        }
        return res + count.into_iter().find(|&n| n == 1).unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome(
                "zeusnilemacaronimaisanitratetartinasiaminoracamelinsuez".to_string()
            ),
            55
        );
    }
}
