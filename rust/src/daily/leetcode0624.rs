// 624. Maximum Distance in Arrays
pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut prev_min = arrays[0][0];
        let mut prev_max = arrays[0][arrays[0].len() - 1];
        let mut cur_diff = 0;

        for arr in arrays.into_iter().skip(1) {
            cur_diff = cur_diff.max(prev_min.abs_diff(arr[arr.len() - 1]) as i32);
            cur_diff = cur_diff.max(prev_max.abs_diff(arr[0]) as i32);
            prev_min = prev_min.min(arr[0]);
            prev_max = prev_max.max(arr[arr.len() - 1]);
        }
        cur_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4
        )
    }
}
