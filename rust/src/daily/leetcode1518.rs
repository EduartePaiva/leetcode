// 1518. Water Bottles
pub struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = 0;
        let mut left = 0;
        while num_bottles > 0 {
            res += num_bottles;
            left += num_bottles % num_exchange;
            num_bottles /= num_exchange;
            if left >= num_exchange {
                left -= num_exchange;
                num_bottles += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }
}
