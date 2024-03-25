// 442. Find All Duplicates in an Array
pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..nums.len() {
            let cur = nums[i].abs() - 1;
            let point_to = nums[cur as usize];
            if point_to.is_negative() {
                res.push(cur + 1);
            } else {
                nums[cur as usize] = -point_to;
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
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }
}
