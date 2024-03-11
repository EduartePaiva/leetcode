// 349. Intersection of Two Arrays
pub struct Solution;
impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut res = vec![];

        let (mut n1, mut n2) = (0, 0);
        while n1 < nums1.len() && n2 < nums2.len() {
            if nums1[n1] < nums2[n2] {
                while n1 < nums1.len() && nums1[n1] < nums2[n2] {
                    n1 += 1;
                }
            } else if nums2[n2] < nums1[n1] {
                while n2 < nums2.len() && nums1[n1] > nums2[n2] {
                    n2 += 1;
                }
            } else {
                let cur_num = nums1[n1];
                res.push(cur_num);
                while n1 < nums1.len() && nums1[n1] == cur_num {
                    n1 += 1;
                }
                while n2 < nums2.len() && nums2[n2] == cur_num {
                    n2 += 1;
                }
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
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::intersection(vec![1, 2], vec![1, 1]), vec![1])
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::intersection(vec![2, 1], vec![1, 2]), vec![1, 2])
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        )
    }
}
