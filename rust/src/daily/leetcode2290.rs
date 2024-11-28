// 2290. Minimum Obstacle Removal to Reach Corner
pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        // min_heap + bfs
        let mut deq: VecDeque<(i32, usize, usize)> = VecDeque::new();
        let rows = grid.len();
        let cols = grid[0].len();
        let row_dest = rows - 1;
        let col_dest = cols - 1;
        deq.push_back((grid[0][0], 0, 0));
        grid[0][0] = 2;
        while let Some((steps, row, col)) = deq.pop_front() {
            if row == row_dest && col == col_dest {
                return steps;
            }
            if row > 0 && grid[row - 1][col] != 2 {
                if grid[row - 1][col] == 1 {
                    deq.push_back((steps + 1, row - 1, col));
                } else {
                    deq.push_front((steps, row - 1, col));
                }
                grid[row - 1][col] = 2;
            }
            if row < rows - 1 && grid[row + 1][col] != 2 {
                if grid[row + 1][col] == 1 {
                    deq.push_back((steps + 1, row + 1, col));
                } else {
                    deq.push_front((steps, row + 1, col));
                }
                grid[row + 1][col] = 2;
            }
            if col > 0 && grid[row][col - 1] != 2 {
                if grid[row][col - 1] == 1 {
                    deq.push_back((steps + 1, row, col - 1));
                } else {
                    deq.push_front((steps, row, col - 1));
                }
                grid[row][col - 1] = 2;
            }
            if col < cols - 1 && grid[row][col + 1] != 2 {
                if grid[row][col + 1] == 1 {
                    deq.push_back((steps + 1, row, col + 1));
                } else {
                    deq.push_front((steps, row, col + 1));
                }
                grid[row][col + 1] = 2;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            2
        )
    }
}
