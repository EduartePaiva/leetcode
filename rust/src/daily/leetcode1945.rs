// 1945. Sum of Digits of String After Convert
pub struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        fn compact_number(mut num: i32) -> i32 {
            let mut r = 0;
            while num > 0 {
                r += num % 10;
                num /= 10;
            }
            r
        }
        let mut res = 0;
        for b in s.as_bytes() {
            res += compact_number((b - b'a') as i32 + 1);
        }
        for _ in 1..k {
            res = compact_number(res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6)
    }
}
