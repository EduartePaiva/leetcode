// 1249. Minimum Remove to Make Valid Parentheses
pub struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut res = vec![];
        let mut open_cnt = 0;
        let mut close_cnt = 0;

        //don't allow more close than open
        for c in s.chars() {
            match c {
                ')' => {
                    if open_cnt > close_cnt {
                        close_cnt += 1;
                        res.push(c)
                    }
                }
                '(' => {
                    open_cnt += 1;
                    res.push(c)
                }
                _ => res.push(c),
            }
        }

        //remove excessive opening (
        let res: Vec<_> = res
            .into_iter()
            .rev()
            .filter(|c| {
                if open_cnt > close_cnt && *c == '(' {
                    open_cnt -= 1;
                    false
                } else {
                    true
                }
            })
            .collect();

        res.into_iter().rev().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de".to_string()
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_remove_to_make_valid("())()(((".to_string()),
            "()()".to_string()
        );
    }
}
