// 476. Number Complement
pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut num_clone = num;
        let mut fill = 0;
        while num_clone > 0 {
            fill <<= 1;
            fill += 1;
            num_clone >>= 1;
        }
        num ^ fill
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_complement(5), 2)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_complement(2), 1)
    }
}
