// 857. Minimum Cost to Hire K Workers
pub struct Solution;

use std::collections::BinaryHeap;
#[derive(Clone, Copy)]
struct Rate {
    numerator: i32,
    denominator: i32,
}

impl Ord for Rate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}

impl PartialOrd for Rate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rate {
    fn eq(&self, other: &Self) -> bool {
        (self.numerator * other.denominator) == (self.denominator * other.numerator)
    }
}

impl Eq for Rate {}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let rates: Vec<Rate> = quality
            .iter()
            .zip(wage.iter())
            .map(|(denominator, numerator)| Rate {
                numerator: *numerator,
                denominator: *denominator,
            })
            .collect();
        let mut rates = quality
            .into_iter()
            .zip(rates.into_iter())
            .collect::<Vec<_>>();
        rates.sort_unstable_by_key(|v| v.1);
        // until now it works

        let mut quality_heap = BinaryHeap::with_capacity(k);
        let mut cur_quality = 0;
        for i in 0..k {
            quality_heap.push(rates[i].0);
            cur_quality += rates[i].0;
        }
        let mut result = cur_quality as f64 / rates[k - 1].1.denominator as f64
            * rates[k - 1].1.numerator as f64;

        for i in k..rates.len() {
            cur_quality -= quality_heap.pop().unwrap();
            quality_heap.push(rates[i].0);
            cur_quality += rates[i].0;
            result = result.min(
                cur_quality as f64 / rates[i].1.denominator as f64 * rates[i].1.numerator as f64,
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
            105.0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3),
            30.666666666666668
        );
    }
}
