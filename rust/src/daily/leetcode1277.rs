// 1277. Count Square Submatrices with All Ones
pub struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let row = matrix.len();
        let col = matrix[0].len();
        fn count_squares(
            matrix: &Vec<Vec<i32>>,
            r_start: usize,
            c_start: usize,
            r_end: usize,
            c_end: usize,
        ) -> i32 {
            for c in c_start..c_end {
                if matrix[r_end - 1][c] == 0 {
                    return 0;
                }
            }
            for r in r_start..r_end {
                if matrix[r][c_end - 1] == 0 {
                    return 0;
                }
            }
            if r_end < matrix.len() && c_end < matrix[0].len() {
                return 1 + count_squares(matrix, r_start, c_start, r_end + 1, c_end + 1);
            }
            return 1;
        }

        for r in 0..row {
            for c in 0..col {
                res += count_squares(&matrix, r, c, r + 1, c + 1);
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
            Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
            15
        );
    }
}
