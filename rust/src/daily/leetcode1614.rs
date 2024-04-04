// 1614. Maximum Nesting Depth of the Parentheses
pub struct Solution;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        //naive stack solution
        let mut stack = 0;
        let mut max_depth = 0;

        for c in s.chars() {
            if c == '(' {
                stack += 1;
                max_depth = max_depth.max(stack);
            } else if c == ')' {
                stack -= 1;
            }
        }
        max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }
}
