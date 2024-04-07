// 678. Valid Parenthesis String
pub struct Solution;
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left_min = 0;
        let mut left_max = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    left_min += 1;
                    left_max += 1;
                }
                ')' => {
                    left_min -= 1;
                    left_max -= 1;
                }
                _ => {
                    left_max += 1;
                    left_min -= 1;
                }
            }
            if left_max < 0 {
                return false;
            }
            if left_min < 0 {
                left_min += 1;
            }
        }

        left_min == 0 || left_max == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::check_valid_string(String::from("()")), true);
    }
}
