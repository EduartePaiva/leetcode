// 1598. Crawler Log Folder
pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for log in logs {
            match log.as_str() {
                "./" => (),
                "../" => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => depth += 1,
            }
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
    }
}
