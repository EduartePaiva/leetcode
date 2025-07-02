// 2434. Using a Robot to Print the Lexicographically Smallest String
pub struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut freq = vec![0; 26];
        let mut stack = Vec::new();
        let mut result = String::new();

        for c in s.as_bytes() {
            freq[(c - b'a') as usize] += 1;
        }

        fn has_smaller(top: usize, freq: &Vec<i32>) -> bool {
            (0..top).any(|i| freq[i] > 0)
        }

        for &b in s.as_bytes() {
            let ch = (b - b'a') as usize;
            freq[ch] -= 1;
            stack.push(b);

            while let Some(last) = stack.last() {
                if has_smaller((last - b'a') as usize, &freq) {
                    break;
                }
                result.push(stack.pop().unwrap() as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::robot_with_string("zza".to_string()),
            "azz".to_string()
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::robot_with_string("bac".to_string()),
            "abc".to_string()
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::robot_with_string("bydizfve".to_string()),
            "bdevfziy".to_string()
        )
    }
}
