// 1945. Sum of Digits of String After Convert
pub struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut stack: Vec<i32> = vec![];

        fn separate_digits(mut digit: i32) -> Vec<i32> {
            let mut res = vec![];
            while digit > 0 {
                res.push(digit % 10);
                digit /= 10;
            }
            res
        }
        for b in s.as_bytes() {
            stack.extend(separate_digits((b - b'a') as i32 + 1));
        }
        for _ in 1..k {
            let sum: i32 = stack.iter().sum();
            stack = separate_digits(sum);
        }
        stack.into_iter().sum()
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
