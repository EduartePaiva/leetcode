// 1636. Sort Array by Increasing Frequency
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }
        // (number/frequency)
        let mut frequency: Vec<_> = count.into_iter().collect();
        frequency.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
        println!("{:?}", frequency);
        let mut res = vec![];
        for (num, cnt) in frequency {
            res.extend((0..cnt).map(|_| num));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        )
    }
}
