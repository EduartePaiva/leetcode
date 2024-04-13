// 85. Maximal Rectangle

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<u32>) -> u32 {
        let length = heights.len();
        let mut stack: Vec<(usize, u32)> = vec![];
        let mut max_area = 0;

        for (i, height) in heights.into_iter().enumerate() {
            let mut start = i;
            while !stack.is_empty() && stack.last().unwrap().1 > height {
                let (index, val) = stack.pop().unwrap();
                let area = (i - index) as u32 * val;
                max_area = max_area.max(area);
                start = index;
            }
            stack.push((start, height));
        }

        while let Some((index, val)) = stack.pop() {
            let area = (length - index) as u32 * val;
            max_area = max_area.max(area);
        }

        max_area
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        //convert to a matrix of u32
        let mut matrix: Vec<Vec<u32>> = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|digit| if digit == '0' { 0 } else { 1 })
                    .collect()
            })
            .collect();

        // mutate the matrix adding the previous rows
        for row in 1..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] != 0 {
                    matrix[row][col] += matrix[row - 1][col];
                }
            }
        }
        // now do the previous question largest rectangle area for each row.
        let mut max_rectangle = 0;
        for row in matrix {
            max_rectangle = max_rectangle.max(Solution::largest_rectangle_area(row));
        }

        max_rectangle as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];

        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }
}
