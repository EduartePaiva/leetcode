// 1544. Make The String Great
pub struct Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            if let Some(last) = stack.last() {
                if last.to_ascii_lowercase() == c.to_ascii_lowercase()
                    && ((last.is_ascii_uppercase() && c.is_ascii_lowercase())
                        || (last.is_ascii_lowercase() && c.is_ascii_uppercase()))
                {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }

        stack.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::make_good("leEeetcode".to_string()),
            "leetcode".to_string()
        );
    }
}
