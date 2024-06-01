// 3110. Score of a String
pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut sum = 0_i32;

        for i in 0..s.len() - 1 {
            sum += s[i].abs_diff(s[i + 1]) as i32;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    }
}
