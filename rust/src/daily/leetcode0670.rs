// 670. Maximum Swap
pub struct Solution;

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        // n^2 is fine
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        digits.reverse();

        for i in 0..digits.len() {
            let cur_n = digits[i];
            let mut max_v = i32::MIN;
            let mut to_swap = i;
            for j in (i + 1..digits.len()).rev() {
                if max_v < digits[j] {
                    to_swap = j;
                    max_v = digits[j];
                }
            }
            if max_v > cur_n {
                digits[to_swap] = cur_n;
                digits[i] = max_v;
                break;
            }
        }

        let mut res = 0;
        for i in 0..digits.len() {
            res += digits[i] * 10_i32.pow((digits.len() - i - 1) as u32);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }
}
