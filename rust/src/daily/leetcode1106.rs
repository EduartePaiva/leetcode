// 1106. Parsing A Boolean Expression
pub struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        // expression[i] is one following characters: '(', ')', '&', '|', '!', 't', 'f', and ','.
        let mut i = 0;
        let s = expression.as_bytes();
        fn helper(i: &mut usize, s: &[u8]) -> bool {
            let c = s[*i];
            *i += 1;
            match c {
                b't' => return true,
                b'f' => return false,
                b'!' => {
                    *i += 1;
                    let res = !helper(i, s);
                    *i += 1;
                    return res;
                }
                _ => (),
            }
            let mut children = Vec::new();
            *i += 1;
            while s[*i] != b')' {
                if s[*i] != b',' {
                    children.push(helper(i, s));
                } else {
                    *i += 1;
                }
            }
            *i += 1;
            if c == b'&' {
                return children.into_iter().all(|x| x);
            }
            if c == b'|' {
                return children.into_iter().any(|x| x);
            }
            panic!("this path is unreachable");
        }
        helper(&mut i, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::parse_bool_expr("&(|(f))".to_string()), false);
    }
}
