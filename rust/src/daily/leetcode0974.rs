// 974. Subarray Sums Divisible by K
pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; k as usize];
        count[0] = 1;
        let mut sum = 0;
        for n in nums {
            sum += n;
            let modulo = if (sum % k).is_negative() {
                (sum % k) + k
            } else {
                sum % k
            } as usize;
            count[modulo] += 1;
        }
        count
            .into_iter()
            .fold(0, |prev, n| prev + ((n - 1) * n) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
    }
}
