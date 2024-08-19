//650. 2 Keys Keyboard
pub struct Solution;

impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let mut res = 0;
        fn get_div(num: i32) -> i32 {
            for n in 2..=num {
                if num % n == 0 {
                    return n;
                }
            }
            num
        }
        while n > 1 {
            let to_divide = get_div(n);
            res += to_divide;
            n /= to_divide;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_steps(3), 3);
    }
}
