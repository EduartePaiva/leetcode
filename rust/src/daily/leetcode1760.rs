// 1760. Minimum Limit of Balls in a Bag
pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut min = 1;
        let mut max = *nums.iter().max().unwrap();

        fn can_divide(nums: &Vec<i32>, max_operations: i32, max_balls: u32) -> bool {
            let mut ops = 0;
            for n in nums {
                ops += (*n as u32).div_ceil(max_balls) as i32 - 1;
                if ops > max_operations {
                    return false;
                }
            }
            true
        }

        while min <= max {
            let mid = (min + max) / 2;

            if can_divide(&nums, max_operations, mid as u32) {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_size(vec![9], 2), 3)
    }
}
