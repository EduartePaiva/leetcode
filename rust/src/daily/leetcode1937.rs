// 1937. Maximum Number of Points with Cost
pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let rows = points.len();
        let cols = points[0].len();
        let mut prev_row: Vec<i64> = points[points.len() - 1].iter().map(|&x| x as i64).collect();

        for r in (0..rows - 1).rev() {
            let mut cur_row = vec![i64::MIN; cols];

            // from left  to right
            let mut left = i64::MIN;
            for c in 0..cols {
                left = left.max(prev_row[c]);
                cur_row[c] = cur_row[c].max(left);
                left -= 1;
            }
            // from right  to left
            let mut right = i64::MIN;
            for c in (0..cols).rev() {
                right = right.max(prev_row[c]);
                cur_row[c] = cur_row[c].max(right);
                right -= 1;
            }
            for c in 0..cols {
                cur_row[c] += points[r][c] as i64;
            }
            prev_row = cur_row;
        }

        prev_row.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]),
            9
        )
    }
}
