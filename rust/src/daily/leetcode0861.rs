// 861. Score After Flipping Matrix
pub struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        // step 1, toggle every row that the first value is 0.
        // step 2, toggle every column except first, that have more 0 than 1.
        // step 3, calculate the sum of the binary representation.
        let rows = grid.len();
        let cols = grid[0].len();

        // step 1.
        for r in 0..rows {
            if grid[r][0] == 0 {
                for c in 0..cols {
                    grid[r][c] = if grid[r][c] == 0 { 1 } else { 0 };
                }
            }
        }

        //step 2.
        for c in 1..cols {
            //stp 2.1 sum up the col
            let mut sm = 0;
            for r in 0..rows {
                sm += grid[r][c];
            }
            // step 2.2 flip the col
            if sm <= (rows as i32 / 2) {
                for r in 0..rows {
                    grid[r][c] = if grid[r][c] == 0 { 1 } else { 0 };
                }
            }
        }

        // step 3.
        let mut res = 0;
        for r in 0..rows {
            let mut cur_num = 1 << (cols - 1);
            for c in 1..cols {
                if grid[r][c] == 1 {
                    cur_num += 1 << (cols - 1 - c);
                }
            }
            res += cur_num;
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
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
    }
}
