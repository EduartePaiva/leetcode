// 2064. Minimized Maximum of Products Distributed to Any Store
pub struct Solution;

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let sum: i64 = quantities.iter().map(|v| *v as i64).sum();
        let mut min = (sum / n as i64) as i32 + if sum % n as i64 != 0 { 1 } else { 0 };
        let mut max = i32::MAX;

        while min <= max {
            let mid = (max - min) / 2 + min;
            let mut new_n = n;
            for num in &quantities {
                new_n -= num / mid;
                new_n -= if num % mid != 0 { 1 } else { 0 };
                if new_n < 0 {
                    break;
                }
            }
            if new_n >= 0 {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 7]), 4);
    }
}
