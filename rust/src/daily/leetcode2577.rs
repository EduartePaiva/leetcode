// 2577. Minimum Time to Visit a Cell In a Grid
pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn minimum_time(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let mut heap: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        // (time, r, c)
        let row_dest = grid.len() - 1;
        let col_dest = grid[0].len() - 1;
        heap.push((Reverse(0), 0, 0));
        grid[0][0] = -1;
        while let Some((Reverse(time), row, col)) = heap.pop() {
            if row_dest == row as usize && col_dest == col as usize {
                return time;
            }
            for (r, c) in [
                (row + 1, col),
                (row, col + 1),
                (row - 1, col),
                (row, col - 1),
            ] {
                if r >= 0
                    && c >= 0
                    && r < grid.len() as i32
                    && c < grid[0].len() as i32
                    && grid[r as usize][c as usize] != -1
                {
                    let diff = grid[r as usize][c as usize] - time;
                    let next_time = (grid[r as usize][c as usize]
                        + if diff % 2 == 0 { 1 } else { 0 })
                    .max(time + 1);
                    heap.push((Reverse(next_time), r, c));
                    grid[r as usize][c as usize] = -1;
                }
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
            Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
            7
        )
    }
}
