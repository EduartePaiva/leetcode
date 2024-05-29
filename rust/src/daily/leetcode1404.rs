// 1404. Number of Steps to Reduce a Number in Binary Representation to One
pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = VecDeque::from_iter(s.chars());

        let mut steps = 0;
        while s.len() > 1 {
            let lst = s.len() - 1;
            if s[lst] == '1' {
                let mut add_first = true;
                for i in (0..=lst).rev() {
                    if s[i] == '0' {
                        add_first = false;
                        s[i] = '1';
                        break;
                    } else {
                        s[i] = '0';
                    }
                }
                if add_first {
                    s.push_front('1');
                }
            } else {
                s.pop_back();
            }
            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
    }
}
