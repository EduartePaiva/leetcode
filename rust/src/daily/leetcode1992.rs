// 1992. Find All Groups of Farmland

pub struct Solution;

impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        for r in 0..land.len() {
            for c in 0..land[0].len() {
                if land[r][c] == 1 {
                    let (bottom, right) = get_right_bottom_and_clean(&mut land, r, c);
                    res.push(vec![r as i32, c as i32, bottom, right]);
                }
            }
        }

        fn get_right_bottom_and_clean(
            land: &mut Vec<Vec<i32>>,
            row: usize,
            col: usize,
        ) -> (i32, i32) {
            let mut r = row;
            let mut c = col;

            while r < land.len() && land[r][col] == 1 {
                c = col;
                while c < land[0].len() && land[r][c] == 1 {
                    land[r][c] = 0;
                    c += 1;
                }
                r += 1;
            }

            return (r as i32 - 1, c as i32 - 1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let land = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]];
        let output = vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]];
        assert_eq!(Solution::find_farmland(land), output);
    }
}
