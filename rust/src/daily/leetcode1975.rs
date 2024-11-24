// 1975. Maximum Matrix Sum
pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut is_odd = false;
        let mut min_negative = i32::MAX;

        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                sum += matrix[r][c].abs() as i64;
                min_negative = min_negative.min(matrix[r][c].abs());
                if matrix[r][c] < 0 {
                    is_odd = !is_odd;
                }
            }
        }
        if is_odd {
            sum - min_negative as i64 * 2
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![-1, 0, -1], vec![-2, 1, 3], vec![3, 2, 2]]),
            15
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![2, 9, 3], vec![5, 4, -4], vec![1, 7, 1]]),
            34
        )
    }
}
