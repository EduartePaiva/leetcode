// Length of Last Word

pub struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().rev().next().unwrap().len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
}
