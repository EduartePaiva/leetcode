// 1137. N-th Tribonacci Number
pub struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;

        match n {
            0 => return t0,
            1 => return t1,
            2 => return t2,
            _ => (),
        }

        for _ in 3..=n {
            let bk_t2 = t2;
            t2 += t0 + t1;
            t0 = t1;
            t1 = bk_t2;
        }

        t2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
