// 3068. Find the Maximum Sum of Node Values
pub struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut max_negative = i32::MIN;
        let mut min_positive = i32::MAX;
        let mut count_positives = 0;
        for num in nums {
            let delta = (num ^ k) - num;
            if delta.is_positive() {
                count_positives += 1;
                min_positive = min_positive.min(delta);
            } else {
                max_negative = max_negative.max(delta);
            }

            sum += if delta.is_positive() {
                num + delta
            } else {
                num
            } as i64;
        }

        if count_positives % 2 != 0 {
            if min_positive + max_negative > 0 {
                sum += max_negative as i64;
            } else {
                sum -= min_positive as i64;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_value_sum(vec![1, 2, 3], 3, vec![vec![0, 1], vec![0, 2]]),
            6
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_value_sum(
                vec![24, 78, 1, 97, 44],
                6,
                vec![vec![0, 2], vec![1, 2], vec![4, 2], vec![3, 4]]
            ),
            260
        );
    }
}
