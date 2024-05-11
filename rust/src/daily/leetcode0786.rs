// 786. K-th Smallest Prime Fraction

pub struct Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        // n^2 solution
        let mut fractions: Vec<(i32, i32)> = vec![];
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                fractions.push((arr[i], arr[j]));
            }
        }
        fractions.sort_unstable_by(|a, b| (a.0 * b.1).cmp(&(a.1 * b.0)));
        // println!("{:?}", fractions);
        Vec::from([fractions[(k - 1) as usize].0, fractions[(k - 1) as usize].1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3),
            vec![2, 5]
        );
    }
}
