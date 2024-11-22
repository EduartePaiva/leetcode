// 1072. Flip Columns For Maximum Number of Equal Rows
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        // most common pattern
        let mut patterns: HashMap<String, i32> = HashMap::new();

        for row in matrix {
            let mut pattern = String::new();
            pattern.push('*');
            for i in 1..row.len() {
                if row[i - 1] != row[i] {
                    pattern.push('|');
                }
                pattern.push('*');
            }
            *patterns.entry(pattern).or_insert(0) += 1;
        }
        patterns.into_values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]),
            1
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
            2
        )
    }
}
