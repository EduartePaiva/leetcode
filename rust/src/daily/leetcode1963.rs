// 1963. Minimum Number of Swaps to Make the String Balanced
pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        // every swap can deal 4?
        let mut stack: Vec<u8> = vec![];
        for c in s.bytes() {
            let last_v = if let Some(&v) = stack.last() {
                v
            } else {
                stack.push(c);
                continue;
            };
            if last_v == b'[' && c == b']' {
                stack.pop();
                continue;
            }
            stack.push(c);
        }
        stack.len().div_ceil(4) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    }
}
