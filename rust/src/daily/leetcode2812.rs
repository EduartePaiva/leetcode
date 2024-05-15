// 2812. Find the Safest Path in a Grid

pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        // find the thieves
        // do a bfs in the grid from the thieves
        // it can either go to right, or down, it'll prioritize the safest option.
        // return the min value found - 1

        let mut thieves: Vec<[usize; 2]> = vec![];
        let (rows, cols) = (grid.len(), grid.len());

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    thieves.push([r, c]);
                }
            }
        }

        let mut deque = VecDeque::from(thieves);

        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let [r, c] = deque.pop_front().unwrap();
                // check the four directions of thief
                let next_factor = grid[r][c] + 1;
                if r + 1 < grid.len() && (grid[r + 1][c] == 0 || grid[r + 1][c] > next_factor) {
                    grid[r + 1][c] = next_factor;
                    deque.push_back([r + 1, c]);
                }
                if c + 1 < grid.len() && (grid[r][c + 1] == 0 || grid[r][c + 1] > next_factor) {
                    grid[r][c + 1] = next_factor;
                    deque.push_back([r, c + 1]);
                }
                if r > 0 && (grid[r - 1][c] == 0 || grid[r - 1][c] > next_factor) {
                    grid[r - 1][c] = next_factor;
                    deque.push_back([r - 1, c]);
                }
                if c > 0 && (grid[r][c - 1] == 0 || grid[r][c - 1] > next_factor) {
                    grid[r][c - 1] = next_factor;
                    deque.push_back([r, c - 1]);
                }
            }
        }

        for i in 0..grid.len() {
            println!("{:?}", grid[i]);
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
    }
}
