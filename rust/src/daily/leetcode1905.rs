// 1905. Count Sub Islands
pub struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        fn is_sub_island(
            grid1: &Vec<Vec<i32>>,
            grid2: &mut Vec<Vec<i32>>,
            r: usize,
            c: usize,
        ) -> bool {
            if grid2[r][c] == 0 {
                return true;
            }
            if grid1[r][c] == 0 {
                return false;
            }
            grid2[r][c] = 0;
            let up = if r > 0 && grid2[r - 1][c] == 1 {
                is_sub_island(grid1, grid2, r - 1, c)
            } else {
                true
            };
            let down = if r + 1 < grid2.len() && grid2[r + 1][c] == 1 {
                is_sub_island(grid1, grid2, r + 1, c)
            } else {
                true
            };
            let right = if c + 1 < grid2[0].len() && grid2[r][c + 1] == 1 {
                is_sub_island(grid1, grid2, r, c + 1)
            } else {
                true
            };
            let left = if c > 0 && grid2[r][c - 1] == 1 {
                is_sub_island(grid1, grid2, r, c - 1)
            } else {
                true
            };

            up && down && left && right
        }
        let mut res = 0;
        for r in 0..grid1.len() {
            for c in 0..grid1[0].len() {
                if grid2[r][c] == 1 && is_sub_island(&grid1, &mut grid2, r, c) {
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
            Solution::count_sub_islands(
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 1, 1]
                ],
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 0, 1, 1, 1],
                    vec![0, 1, 0, 0, 0],
                    vec![1, 0, 1, 1, 0],
                    vec![0, 1, 0, 1, 0]
                ]
            ),
            3
        )
    }
}
