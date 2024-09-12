// 287. Find the Duplicate Number

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut turtle = nums[0];
        let mut hare = nums[0];

        loop {
            turtle = nums[turtle as usize];
            hare = nums[nums[hare as usize] as usize];
            if turtle == hare {
                break;
            }
        }
        hare = nums[0];
        while turtle != hare {
            turtle = nums[turtle as usize];
            hare = nums[hare as usize];
        }
        turtle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_duplicate(vec![3, 3, 3, 3, 3]), 3);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_duplicate(vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1]),
            //----------------------------0  1  2  3  4  5  6  7  8  9
            9
        );
    }
}
