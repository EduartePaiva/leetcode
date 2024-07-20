// 1605. Find Valid Matrix Given Row and Column Sums
pub struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let rows = row_sum.len();
        let cols = col_sum.len();
        let mut res: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
        for r in 0..rows {
            res[r][0] = row_sum[r];
        }
        let mut cur_col_sum: i64 = row_sum.into_iter().map(|x| x as i64).sum();
        for c in 0..cols - 1 {
            let next = cur_col_sum - col_sum[c] as i64;
            cur_col_sum = next;
            for r in 0..rows {
                if cur_col_sum == 0 {
                    break;
                }
                let to_subtract = cur_col_sum.min(res[r][c] as i64);
                res[r][c] -= to_subtract as i32;
                res[r][c + 1] = to_subtract as i32;
                cur_col_sum -= to_subtract;
            }
            cur_col_sum = next;
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
            Solution::restore_matrix(vec![3, 8], vec![4, 7]),
            vec![[0, 3], [4, 4]]
        );
    }
}
