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
                let mut visit = [0; 10];
                let mut double = false;
                for i in r..r + 3 {
                    for j in c..c + 3 {
                        match grid[i][j] {
                            1..=9 => {
                                visit[grid[i][j] as usize] += 1;
                                if visit[grid[i][j] as usize] > 1 {
                                    double = true;
                                }
                            }
                            _ => {
                                double = true;
                                break;
                            }
                        }
                    }
                    if double {
                        break;
                    }
                }
                if !double
                    && grid[r][c] + grid[r + 1][c] + grid[r + 2][c] == 15
                    && grid[r][c + 1] + grid[r + 1][c + 1] + grid[r + 2][c + 1] == 15
                    && grid[r][c + 2] + grid[r + 1][c + 2] + grid[r + 2][c + 2] == 15
                    && grid[r][c] + grid[r][c + 1] + grid[r][c + 2] == 15
                    && grid[r + 1][c] + grid[r + 1][c + 1] + grid[r + 1][c + 2] == 15
                    && grid[r + 2][c] + grid[r + 2][c + 1] + grid[r + 2][c + 2] == 15
                    && grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2] == 15
                    && grid[r][c + 2] + grid[r + 1][c + 1] + grid[r + 2][c] == 15
                {
                    res += 1;
                }
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
    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_magic_squares_inside(vec![vec![10, 3, 5], vec![1, 6, 11], vec![7, 9, 2]]),
            0
        );
    }
}
