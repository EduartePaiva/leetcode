// 2463. Minimum Total Distance Traveled
pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable_by_key(|k| k[0]);
        let mut factory_positions = vec![];
        for f in factory {
            for _ in 0..f[1] {
                factory_positions.push(f[0]);
            }
        }
        let robot_count = robot.len();
        let factory_count = factory_positions.len();

        let mut dp = vec![vec![0_i64; factory_count + 1]; robot_count + 1];
        for i in 0..robot.len() {
            dp[i][factory_positions.len()] = 1e12 as i64;
        }

        for r in (0..robot_count).rev() {
            for c in (0..factory_count).rev() {
                let assign = robot[r].abs_diff(factory_positions[c]) as i64 + dp[r + 1][c + 1];
                let skip = dp[r][c + 1];
                dp[r][c] = assign.min(skip);
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        )
    }
}
