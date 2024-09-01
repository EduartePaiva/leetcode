// 2022. Convert 1D Array Into 2D Array
pub struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, rows: i32, cols: i32) -> Vec<Vec<i32>> {
        let rows = rows as usize;
        let cols = cols as usize;
        if rows * cols != original.len() {
            return vec![];
        }
        (0..rows)
            .map(|r| Vec::from(&original[r * cols..r * cols + cols]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        )
    }
}
