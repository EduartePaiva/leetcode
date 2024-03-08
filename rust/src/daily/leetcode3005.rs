// 3005. Count Elements With Maximum Frequency
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut frequency = HashMap::new();
        let mut frequency_of_frequency = HashMap::new();
        for num in nums {
            *frequency.entry(num).or_insert(0) += 1;
        }

        for frequency in frequency.into_values() {
            *frequency_of_frequency.entry(frequency).or_insert(0) += frequency;
        }

        let max_key = frequency_of_frequency.keys().max().unwrap();
        *frequency_of_frequency.get(max_key).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_frequency_elements(vec![10, 12, 11, 9, 6, 19, 11]),
            2
        );
    }
}
