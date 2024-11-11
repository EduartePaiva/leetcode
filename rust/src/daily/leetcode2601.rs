// 2601. Prime Subtraction Operation
pub struct Solution;

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        fn is_prime(n: i32) -> bool {
            if n == 1 {
                return false;
            } // 1 is not prime
            if n < 4 {
                return true;
            }; // 2 and 3 are both prime
            if (n % 2) == 0 {
                return false;
            }; // exclude even numbers
            if n < 9 {
                return true;
            }; //we have already excluded 4, 6, and 8.
            if (n % 3) == 0 {
                return false;
            }; // exclude remaining multiples of 3

            let r = (n as f64).sqrt() as i32;
            let mut f = 5;
            while f <= r {
                if (n % f) == 0 {
                    return false;
                };
                if (n % (f + 2)) == 0 {
                    return false;
                };
                f = f + 6;
            }
            return true; // (in all other cases)
        }
        for n in (1..nums[0]).rev() {
            if is_prime(n) {
                nums[0] -= n;
                break;
            }
        }

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                return false;
            }
            let prev = nums[i - 1];
            let max_prime = nums[i] - prev;
            for possible_prime in (1..max_prime).rev() {
                if is_prime(possible_prime) {
                    nums[i] -= possible_prime;
                    break;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::prime_sub_operation(vec![4, 9, 6, 10]), true);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::prime_sub_operation(vec![5, 8, 3]), false);
    }
}
