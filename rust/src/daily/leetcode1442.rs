// 1442. Count Triplets That Can Form Two Arrays of Equal XOR
pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut res = 0;

        for i in 0..n - 1 {
            let mut a = arr[i];
            for k in i + 1..n {
                a ^= arr[k];
                if a == 0 {
                    res += (k - i) as i32;
                }
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
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
    }
}
