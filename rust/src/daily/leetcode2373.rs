// 2373. Largest Local Values in a Matrix
pub struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // let's solve in the most straightforward way

        fn max_around(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
            let mut res = 0;
            let r_start = row - 1;
            let r_end = row + 2;
            let c_start = col - 1;
            let c_end = col + 2;
            for r in r_start..r_end {
                for c in c_start..c_end {
                    res = res.max(grid[r][c]);
                }
            }
            res
        }

        let mut max_local = vec![vec![0; grid.len() - 2]; grid.len() - 2];
        for row in 1..grid.len() - 1 {
            for col in 1..grid.len() - 1 {
                max_local[row - 1][col - 1] = max_around(&grid, row, col);
            }
        }

        max_local
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
    }
}
