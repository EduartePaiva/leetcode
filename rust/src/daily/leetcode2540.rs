// 2540. Minimum Common Value
pub struct Solution;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut n1, mut n2) = (0, 0);

        while n1 < nums1.len() && n2 < nums2.len() {
            if nums1[n1] == nums2[n2] {
                return nums1[n1];
            }
            if nums1[n1] < nums2[n2] {
                n1 += 1;
            } else {
                n2 += 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }
}
