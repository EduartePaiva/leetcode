// 1608. Special Array With X Elements Greater Than or Equal X
pub struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        // binary search between 1 and 1000
        let mut right = 1;
        let mut left = nums.len() as i32;

        while right <= left {
            let half = ((left - right) / 2) + right;
            // check half
            let mut count = 0;
            for &n in &nums {
                if n >= half {
                    count += 1;
                }
            }
            if count == half {
                return half;
            } else if count > half {
                right = half + 1;
            } else {
                left = half - 1;
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
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
    }
}
