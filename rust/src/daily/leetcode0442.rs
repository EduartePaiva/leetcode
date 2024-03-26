// 442. Find All Duplicates in an Array
pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .filter_map(|i| {
                let cur = nums[i].abs() - 1;
                let point_to = nums[cur as usize];
                match point_to.is_negative() {
                    true => Some(cur + 1),
                    false => {
                        nums[cur as usize] = -point_to;
                        None
                    }
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }
}
