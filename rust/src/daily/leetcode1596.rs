// 1568. Minimum Number of Days to Disconnect Island
pub struct Solution;

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        fn fill_island(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) {
            if grid[r][c] == 1 {
                grid[r][c] = 0;

                if r + 1 < grid.len() {
                    fill_island(grid, r + 1, c);
                }
                if r > 0 {
                    fill_island(grid, r - 1, c);
                }
                if c + 1 < grid[0].len() {
                    fill_island(grid, r, c + 1);
                }
                if c > 0 {
                    fill_island(grid, r, c - 1);
                }
            }
        }

        fn find_2_island(mut grid: Vec<Vec<i32>>) -> bool {
            let mut found_island = false;
            for r in 0..grid.len() {
                for c in 0..grid[0].len() {
                    if grid[r][c] == 1 {
                        if found_island {
                            return found_island;
                        }
                        found_island = true;
                        fill_island(&mut grid, r, c);
                    }
                }
            }
            false
        }
        if find_2_island(grid.clone()) {
            return 0;
        }

        let mut cnt = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    cnt += 1;
                    grid[r][c] = 0;
                    if find_2_island(grid.clone()) {
                        return 1;
                    }
                    grid[r][c] = 1;
                }
            }
        }

        if cnt < 2 {
            cnt
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]),
            2
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1]
            ]),
            1
        );
    }
}
