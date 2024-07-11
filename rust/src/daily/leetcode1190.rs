// 1190. Reverse Substrings Between Each Pair of Parentheses
pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut reverse_stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == ')' {
                let mut new_reversed = vec![];
                while let Some(chr) = reverse_stack.pop() {
                    if chr == '(' {
                        break;
                    }
                    new_reversed.push(chr);
                }
                reverse_stack.extend(new_reversed);
            } else {
                reverse_stack.push(c);
            }
        }
        String::from_iter(reverse_stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_parentheses(String::from("(u(love)i)")),
            String::from("iloveu")
        )
    }
}
