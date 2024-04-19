// 200. Number of Islands

pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn expand(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
            if row < 0
                || col < 0
                || row as usize >= grid.len()
                || col as usize >= grid[0].len()
                || grid[row as usize][col as usize] != '1'
            {
                return;
            }
            grid[row as usize][col as usize] = '0';
            expand(grid, row + 1, col);
            expand(grid, row - 1, col);
            expand(grid, row, col + 1);
            expand(grid, row, col - 1);
        }

        let mut num_islands = 0;

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    num_islands += 1;
                    expand(&mut grid, r as i32, c as i32);
                }
            }
        }

        num_islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }
    #[test]
    fn test2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }
}
