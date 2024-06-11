//1122. Relative Sort Array
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        for n in arr1 {
            *count.entry(n).or_insert(0) += 1;
        }
        let mut res = vec![];
        for n in arr2 {
            let key_cnt = *count.get(&n).unwrap_or(&0);
            res.extend((0..key_cnt).map(|_| n));
            count.remove(&n);
        }
        let mut remaining: Vec<i32> = count.keys().map(|k| *k).collect();
        remaining.sort_unstable();
        for n in remaining {
            let key_cnt = *count.get(&n).unwrap_or(&0);
            res.extend((0..key_cnt).map(|_| n));
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
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }
}
