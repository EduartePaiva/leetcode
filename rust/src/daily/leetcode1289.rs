// 1289. Minimum Falling Path Sum II
pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 {
            return grid[0][0];
        }

        let rows = grid.len();
        let cols = grid[0].len();

        fn two_smallest(row: &Vec<i32>) -> [(i32, usize); 2] {
            let mut two_smallest = [(row[0], 0_usize), (row[1], 1_usize)];
            two_smallest.sort_unstable();
            for (i, val) in row.iter().enumerate().skip(2) {
                if *val < two_smallest[1].0 {
                    two_smallest[1] = (*val, i);
                    two_smallest.sort_unstable();
                }
            }
            two_smallest
        }

        let mut prev = two_smallest(&grid[rows - 1]);
        let mut cur = vec![0; cols];
        for r in (0..rows - 1).rev() {
            for c in 0..cols {
                let min_val = if prev[0].1 != c { prev[0].0 } else { prev[1].0 };
                cur[c] = min_val + grid[r][c];
            }
            prev = two_smallest(&cur);
        }

        i32::min(prev[0].0, prev[1].0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            13
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![
                vec![-73, 61, 43, -48, -36],
                vec![3, 30, 27, 57, 10],
                vec![96, -76, 84, 59, -15],
                vec![5, -49, 76, 31, -7],
                vec![97, 91, 61, -46, 67]
            ]),
            -192
        );
    }
}
