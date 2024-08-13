// 40. Combination Sum II
pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates
            .into_iter()
            .filter(|v| v <= &target)
            .collect::<Vec<_>>();
        candidates.sort_unstable();

        let mut res = vec![];

        fn dfs(
            candidates: &Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            cur_nums: &mut Vec<i32>,
            mut i: usize,
            target: i32,
        ) {
            if target == 0 {
                res.push(cur_nums.clone());
                return;
            }
            if i == candidates.len() || target < 0 {
                return;
            }
            // include current
            cur_nums.push(candidates[i]);
            dfs(candidates, res, cur_nums, i + 1, target - candidates[i]);
            cur_nums.pop();

            // skip current
            let cur_n = candidates[i];
            while i < candidates.len() && candidates[i] == cur_n {
                i += 1;
            }
            dfs(candidates, res, cur_nums, i, target);
        }
        let mut cur_nums = vec![];
        dfs(&candidates, &mut res, &mut cur_nums, 0, target);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 2, 5, 6, 7, 10], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }
}
