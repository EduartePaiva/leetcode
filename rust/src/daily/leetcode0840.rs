// 840. Magic Squares In Grid
pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut res = 0;
        //do the most brute force way

        for r in 0..rows - 2 {
            for c in 0..cols - 2 {
                let f_r = grid[r][c] + grid[r + 1][c] + grid[r + 2][c];
                let s_r = grid[r][c + 1] + grid[r + 1][c + 1] + grid[r + 2][c + 1];
                if f_r != s_r {
                    continue;
                }
                let t_r = grid[r][c + 2] + grid[r + 1][c + 2] + grid[r + 2][c + 2];
                if f_r != t_r {
                    continue;
                }
                let f_c = grid[r][c] + grid[r][c + 1] + grid[r][c + 2];
                if f_r != f_c {
                    continue;
                }
                let s_c = grid[r + 1][c] + grid[r + 1][c + 1] + grid[r + 1][c + 2];
                if f_r != s_c {
                    continue;
                }
                let t_c = grid[r + 2][c] + grid[r + 2][c + 1] + grid[r + 2][c + 2];
                if f_r != t_c {
                    continue;
                }
                let d_1 = grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2];
                if f_r != d_1 {
                    continue;
                }
                let d_2 = grid[r][c + 2] + grid[r + 1][c + 1] + grid[r + 2][c];
                if f_r != d_2 {
                    continue;
                }
                res += 1;
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
            Solution::num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2]
            ]),
            1
        );
    }
}
