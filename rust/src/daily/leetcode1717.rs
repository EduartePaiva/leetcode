// 1717. Maximum Score From Removing Substrings
pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, mut x: i32, mut y: i32) -> i32 {
        // can this be greedy?
        //paaadbfbybbbtaa
        // x = 4 = ab, y = 5 = ba
        let first;
        let second;
        if x < y {
            let bkp = x;
            x = y;
            y = bkp;
            first = 'b';
            second = 'a';
        } else {
            first = 'a';
            second = 'b';
        }
        let mut res = 0;
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == second {
                if let Some(&other_char) = stack.last() {
                    if other_char == first {
                        stack.pop();
                        res += x;
                        continue;
                    }
                }
            }
            stack.push(c);
        }
        let mut stack2: Vec<char> = vec![];
        for c in stack {
            if c == first {
                if let Some(&other_char) = stack2.last() {
                    if other_char == second {
                        stack2.pop();
                        res += y;
                        continue;
                    }
                }
            }
            stack2.push(c);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_gain("paaaabdbabfbybbbtaaab".to_string(), 8132, 1776),
            24396
        );
    }
}
