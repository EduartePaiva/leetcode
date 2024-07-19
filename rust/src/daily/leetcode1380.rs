// 1380. Lucky Numbers in a Matrix
pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut res = vec![];
        let mut min_rows = vec![i32::MAX; rows];
        let mut max_cols = vec![i32::MIN; cols];

        for r in 0..rows {
            for c in 0..cols {
                min_rows[r] = min_rows[r].min(matrix[r][c]);
            }
        }
        for c in 0..cols {
            for r in 0..rows {
                max_cols[c] = max_cols[c].max(matrix[r][c]);
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                if min_rows[r] == matrix[r][c] && max_cols[c] == matrix[r][c] {
                    res.push(matrix[r][c]);
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
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
    }
}
