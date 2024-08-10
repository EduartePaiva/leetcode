// 959. Regions Cut By Slashes
pub struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let mut new_grid: Vec<Vec<i32>> = vec![];

        for r in grid {
            let mut row1: Vec<i32> = vec![];
            let mut row2 = vec![];
            let mut row3 = vec![];
            for c in r.chars() {
                match c {
                    ' ' => {
                        row1.extend([0, 0, 0]);
                        row2.extend([0, 0, 0]);
                        row3.extend([0, 0, 0]);
                    }
                    '\\' => {
                        row1.extend([1, 0, 0]);
                        row2.extend([0, 1, 0]);
                        row3.extend([0, 0, 1]);
                    }
                    '/' => {
                        row1.extend([0, 0, 1]);
                        row2.extend([0, 1, 0]);
                        row3.extend([1, 0, 0]);
                    }
                    _ => panic!("this can't happen"),
                }
            }
            new_grid.push(row1);
            new_grid.push(row2);
            new_grid.push(row3);
        }
        fn fill_blank_space(new_grid: &mut Vec<Vec<i32>>, r: usize, c: usize) {
            if new_grid[r][c] == 1 {
                return;
            }
            new_grid[r][c] = 1;
            if r + 1 < new_grid.len() {
                fill_blank_space(new_grid, r + 1, c);
            }
            if r > 0 {
                fill_blank_space(new_grid, r - 1, c);
            }
            if c + 1 < new_grid[0].len() {
                fill_blank_space(new_grid, r, c + 1);
            }
            if c > 0 {
                fill_blank_space(new_grid, r, c - 1);
            }
        }

        let mut res = 0;
        let rows = new_grid.len();
        let cols = new_grid[0].len();
        for r in 0..rows {
            for c in 0..cols {
                if new_grid[r][c] == 0 {
                    res += 1;
                    fill_blank_space(&mut new_grid, r, c);
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
            Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]),
            2
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::regions_by_slashes(vec!["/\\".to_string(), "\\/".to_string()]),
            5
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::regions_by_slashes(vec!["//".to_string(), "/ ".to_string()]),
            3
        );
    }
}
