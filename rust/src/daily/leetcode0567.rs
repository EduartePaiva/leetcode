// 567. Permutation in String
pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut map1 = [0; 26];
        let mut map2 = [0; 26];
        let window_size = s1.len();

        for c in s1.bytes() {
            map1[(c - b'a') as usize] += 1;
        }
        let s2 = s2.as_bytes();
        for i in 0..window_size {
            map2[(s2[i] - b'a') as usize] += 1;
        }
        if map1 == map2 {
            return true;
        }
        for i in window_size..s2.len() {
            map2[(s2[i - window_size] - b'a') as usize] -= 1;
            map2[(s2[i] - b'a') as usize] += 1;
            if map1 == map2 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        )
    }
}
