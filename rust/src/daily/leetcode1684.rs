//1684. Count the Number of Consistent Strings
pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut bit_trick = 0;
        for b in allowed.as_bytes() {
            bit_trick |= 1 << b - b'a';
        }
        let mut res = 0;
        for w in words {
            res += 1;
            for c in w.bytes() {
                if (bit_trick >> (c - b'a')) % 2 == 0 {
                    res -= 1;
                    break;
                }
            }
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
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
    }
}
