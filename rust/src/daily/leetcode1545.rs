// 1545. Find Kth Bit in Nth Binary String
pub struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let k = k as usize;
        let mut s: Vec<bool> = Vec::new();
        s.push(false);
        fn reverse(mut si_minus_1: Vec<bool>) -> Vec<bool> {
            si_minus_1.reverse();
            si_minus_1
        }
        fn invert(mut si_minus_1: Vec<bool>) -> Vec<bool> {
            for i in 0..si_minus_1.len() {
                si_minus_1[i] = !si_minus_1[i];
            }
            si_minus_1
        }
        for _ in 1..n as usize {
            let si_minus_1 = s.clone();
            s.push(true);
            s.extend(reverse(invert(si_minus_1)));
            if s.len() > k {
                break;
            }
        }
        if s[k - 1] {
            '1'
        } else {
            '0'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_kth_bit(5, 24), '0');
    }
}
