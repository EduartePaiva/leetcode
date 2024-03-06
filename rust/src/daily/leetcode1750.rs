// 1750. Minimum Length of String After Deleting Similar Ends
pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let str_slice = s.as_bytes();
        let mut left = 0;
        let mut right = str_slice.len() - 1;

        while left < right {
            if str_slice[left] == str_slice[right] {
                let val_eq = str_slice[left];
                left += 1;
                right -= 1;

                while left < right && val_eq == str_slice[left] {
                    left += 1;
                }
                while left <= right && val_eq == str_slice[right] {
                    right -= 1;
                }
            } else {
                break;
            }
        }
        (right + 1 - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_length(String::from("ca")), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_length(String::from("cabaabac")), 0);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_length(String::from("aabccabba")), 3);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::minimum_length(String::from("abbbbbbbbbbbbbbbbbbba")),
            0
        );
    }
}
