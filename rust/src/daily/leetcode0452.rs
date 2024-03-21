// 452. Minimum Number of Arrows to Burst Balloons

pub struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();

        let mut last_end = points[0][1];
        let mut res = 1;

        for i in 1..points.len() {
            if points[i][0] <= last_end {
                last_end = last_end.min(points[i][1]);
            } else {
                res += 1;
                last_end = points[i][1];
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
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        )
    }
}
