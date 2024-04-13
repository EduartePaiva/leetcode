// 84. Largest Rectangle in Histogram

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        //monotonic stack
        //stack will store it's position and the height
        let length = heights.len();
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut max_area = 0;

        for (i, height) in heights.into_iter().enumerate() {
            let mut start = i;
            while !stack.is_empty() && stack.last().unwrap().1 > height {
                let (index, val) = stack.pop().unwrap();

                let area = (i - index) as i32 * val;
                max_area = max_area.max(area);
                start = index;
            }
            stack.push((start, height));
        }

        while let Some((index, val)) = stack.pop() {
            let area = (length - index) as i32 * val;
            max_area = max_area.max(area);
        }

        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
