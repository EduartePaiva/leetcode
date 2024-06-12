// 75. Sort Colors
pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // let's use bucket sort
        let mut red = 0;
        let mut white = 0;
        let mut blue = 0;

        for n in nums.iter() {
            match n {
                0 => red += 1,
                1 => white += 1,
                2 => blue += 1,
                _ => (),
            }
        }
        let mut pointer = 0;
        for i in pointer..pointer + red {
            nums[i] = 0;
        }
        pointer += red;
        for i in pointer..pointer + white {
            nums[i] = 1;
        }
        pointer += white;
        for i in pointer..pointer + blue {
            nums[i] = 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut vec1 = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut vec1);
        assert_eq!(vec1, vec![0, 0, 1, 1, 2, 2]);
    }
}
