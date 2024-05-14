// 1219. Path with Maximum Gold
pub struct Solution;

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
            if grid[row][col] == 0 {
                return 0;
            }
            let mut max_path = 0;
            let cur_val = grid[row][col];
            grid[row][col] = 0;
            if row > 0 {
                max_path = max_path.max(dfs(grid, row - 1, col));
            }
            if col > 0 {
                max_path = max_path.max(dfs(grid, row, col - 1));
            }
            if row + 1 < grid.len() {
                max_path = max_path.max(dfs(grid, row + 1, col));
            }
            if col + 1 < grid[0].len() {
                max_path = max_path.max(dfs(grid, row, col + 1));
            }

            grid[row][col] = cur_val;
            return cur_val + max_path;
        }

        let mut res = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                res = res.max(dfs(&mut grid, r, c));
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
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        )
    }
}
