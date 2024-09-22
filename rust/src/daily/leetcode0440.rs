// 440. K-th Smallest in Lexicographical Order
pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let k = k as i64;
        let mut cur = 1;
        let mut i = 1;

        let count = |mut cur: i64| -> i64 {
            let mut res = 0;
            let mut nei = cur + 1;
            while cur <= n {
                res += nei.min(n + 1) - cur;
                cur *= 10;
                nei *= 10;
            }
            res
        };

        while i < k {
            let steps = count(cur);
            if i + steps <= k {
                cur += 1;
                i += steps;
            } else {
                cur *= 10;
                i += 1;
            }
        }
        cur as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_number(13, 2), 10)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_kth_number(2, 2), 2)
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_kth_number(10, 3), 2)
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::find_kth_number(100, 90), 9)
    }
    #[test]
    fn test5() {
        assert_eq!(Solution::find_kth_number(1692778, 1545580), 867519)
    }
}
