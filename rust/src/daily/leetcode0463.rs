// 463. Island Perimeter

pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut res = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    res += 4;
                    if r > 0 && grid[r - 1][c] == 1 {
                        res -= 2;
                    }
                    if c > 0 && grid[r][c - 1] == 1 {
                        res -= 2;
                    }
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
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }
}
