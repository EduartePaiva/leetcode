// 78. Subsets
pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets: Vec<Vec<i32>> = Vec::new();
        fn dfs(i: usize, nums: &Vec<i32>, subsets: &mut Vec<Vec<i32>>, mut cur_set: Vec<i32>) {
            if i == nums.len() {
                subsets.push(cur_set);
                return;
            }
            dfs(i + 1, nums, subsets, cur_set.clone());
            cur_set.push(nums[i]);
            dfs(i + 1, nums, subsets, cur_set);
        }
        dfs(0, &nums, &mut subsets, vec![]);
        subsets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3]
            ]
        );
    }
}
