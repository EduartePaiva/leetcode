// 2257. Count Unguarded Cells in the Grid
pub struct Solution;

#[derive(Clone, Copy)]
enum Grid {
    Blocked,
    Free,
    Visited,
}

use Grid::*;
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut occupied = (guards.len() + walls.len()) as i32;
        let rows = m as usize;
        let cols = n as usize;
        let mut grid = vec![vec![Free; cols]; rows];

        for guard in &guards {
            grid[guard[0] as usize][guard[1] as usize] = Blocked;
        }
        for wall in &walls {
            grid[wall[0] as usize][wall[1] as usize] = Blocked;
        }
        for guard in &guards {
            let guard_row = guard[0] as usize;
            let guard_col = guard[1] as usize;

            // north
            for r in (0..guard_row).rev() {
                match grid[r][guard_col] {
                    Blocked => break,
                    Free => {
                        grid[r][guard_col] = Visited;
                        occupied += 1;
                    }
                    Visited => (),
                }
            }

            // south
            for r in guard_row + 1..rows {
                match grid[r][guard_col] {
                    Blocked => break,
                    Free => {
                        grid[r][guard_col] = Visited;
                        occupied += 1;
                    }
                    Visited => (),
                }
            }

            // east
            for c in guard_col + 1..cols {
                match grid[guard_row][c] {
                    Blocked => break,
                    Free => {
                        grid[guard_row][c] = Visited;
                        occupied += 1;
                    }
                    Visited => (),
                }
            }

            // west
            for c in (0..guard_col).rev() {
                match grid[guard_row][c] {
                    Blocked => break,
                    Free => {
                        grid[guard_row][c] = Visited;
                        occupied += 1;
                    }
                    Visited => (),
                }
            }
        }

        m * n - occupied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                vec![vec![0, 1], vec![2, 2], vec![1, 4]]
            ),
            7
        );
    }
}
