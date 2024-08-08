// 885. Spiral Matrix III
pub struct Solution;

enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let matrix_size = (rows * cols) as usize;
        let mut res = vec![];

        let mut r_pos = r_start;
        let mut c_pos = c_start;
        let mut dir = East;
        let mut step = 1;
        while res.len() < matrix_size {
            for _ in 0..2 {
                match dir {
                    East => {
                        if r_pos < rows && r_pos >= 0 {
                            for c in c_pos..c_pos + step {
                                if c >= 0 && c < cols {
                                    res.push(vec![r_pos, c]);
                                }
                            }
                        }
                        c_pos = c_pos + step;
                        dir = South;
                    }
                    South => {
                        if c_pos < cols && c_pos >= 0 {
                            for r in r_pos..r_pos + step {
                                if r >= 0 && r < rows {
                                    res.push(vec![r, c_pos]);
                                }
                            }
                        }
                        r_pos = r_pos + step;
                        dir = West;
                    }
                    West => {
                        if r_pos < rows && r_pos >= 0 {
                            for c in (c_pos - step + 1..=c_pos).rev() {
                                if c >= 0 && c < cols {
                                    res.push(vec![r_pos, c]);
                                }
                            }
                        }
                        c_pos = c_pos - step;
                        dir = North;
                    }
                    North => {
                        if c_pos < cols && c_pos >= 0 {
                            let start = if r_pos - step + 1 > 0 {
                                r_pos - step + 1
                            } else {
                                0
                            };
                            let end = if r_pos < rows { r_pos } else { rows - 1 };
                            for r in (start..=end).rev() {
                                res.push(vec![r, c_pos]);
                            }
                        }
                        r_pos = r_pos - step;
                        dir = East;
                    }
                }
            }
            step += 1;
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
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::spiral_matrix_iii(3, 3, 0, 0),
            [
                [0, 0],
                [0, 1],
                [1, 1],
                [1, 0],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 1],
                [2, 0]
            ]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        )
    }
}
