// 2109. Adding Spaces to a String
pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, mut spaces: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut new_s = String::with_capacity(s.len() + spaces.len());
        spaces.reverse();

        for i in 0..s.len() {
            if let Some(&lst) = spaces.last() {
                if i as i32 == lst {
                    new_s.push(' ');
                    spaces.pop();
                }
            }
            new_s.push(s[i] as char);
        }

        new_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn".to_string()
        )
    }
}
