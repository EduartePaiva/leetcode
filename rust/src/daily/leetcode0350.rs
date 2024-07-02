//350. Intersection of Two Arrays II
pub struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_map = vec![0; 1001];
        for n in nums1 {
            hash_map[n as usize] += 1;
        }
        let mut res = vec![];
        for n in nums2 {
            if hash_map[n as usize] > 0 {
                hash_map[n as usize] -= 1;
                res.push(n);
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
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        )
    }
}
