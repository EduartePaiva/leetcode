// 42. Trapping Rain Water

pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        //sliding window
        //[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut l_height = height[left];
        let mut r_height = height[right];
        let mut water = 0;

        while left < right {
            if l_height > r_height {
                // right height is less than left height
                // shift right to the left
                right -= 1;
                if height[right] < r_height {
                    water += r_height - height[right];
                } else {
                    r_height = height[right];
                }
            } else {
                //left height is less or equal to right height
                // shift left to the right
                left += 1;
                if height[left] < l_height {
                    water += l_height - height[left];
                } else {
                    l_height = height[left];
                }
            }
        }

        water
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
