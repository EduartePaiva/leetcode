// 2696. Minimum String Length After Removing Substrings
pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        // stack problem
        let mut stack: Vec<u8> = vec![];

        for c in s.bytes() {
            match c {
                b'B' => {
                    if let Some(&lst) = stack.last() {
                        if lst == b'A' {
                            stack.pop();
                            continue;
                        }
                    }
                }
                b'D' => {
                    if let Some(&lst) = stack.last() {
                        if lst == b'C' {
                            stack.pop();
                            continue;
                        }
                    }
                }
                _ => (),
            }
            stack.push(c);
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
    }
}
