// 921. Minimum Add to Make Parentheses Valid
pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open = 0;
        let mut excess_close = 0;

        for c in s.bytes() {
            if c == b'(' {
                open += 1;
            } else {
                if open == 0 {
                    excess_close += 1;
                } else {
                    open -= 1;
                }
            }
        }

        open + excess_close
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }
}
