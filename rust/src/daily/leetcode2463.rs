// 2463. Minimum Total Distance Traveled
pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable_by_key(|k| k[0]);
        let mut unpacked_factory = vec![];
        for f in factory {
            for _ in 0..f[1] {
                unpacked_factory.push(f[0]);
            }
        }
        let mut cache = vec![vec![-1_i64; unpacked_factory.len()]; robot.len()];
        fn dfs(
            robot: &Vec<i32>,
            unpacked_factory: &Vec<i32>,
            rbt_i: usize,
            fct_i: usize,
            cache: &mut Vec<Vec<i64>>,
        ) -> i64 {
            if rbt_i == robot.len() {
                return 0;
            }
            if fct_i == unpacked_factory.len() {
                return -1;
            }
            if cache[rbt_i][fct_i] != -1 {
                return cache[rbt_i][fct_i];
            }
            let opt_1 = dfs(robot, unpacked_factory, rbt_i, fct_i + 1, cache);
            let opt_2 = unpacked_factory[fct_i].abs_diff(robot[rbt_i]) as i64
                + dfs(robot, unpacked_factory, rbt_i + 1, fct_i + 1, cache);
            let res = if opt_1.min(opt_2) != -1 {
                opt_1.min(opt_2)
            } else {
                opt_1.max(opt_2)
            };
            cache[rbt_i][fct_i] = res;
            res
        }

        dfs(&robot, &unpacked_factory, 0, 0, &mut cache)
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
