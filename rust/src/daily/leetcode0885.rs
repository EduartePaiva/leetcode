// 885. Spiral Matrix III
pub struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let matrix_size = (rows * cols) as usize;
        let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut res = vec![];
        let mut r = r_start;
        let mut c = c_start;
        let mut steps = 1;
        let mut i = 0;
        while res.len() < matrix_size {
            for _ in 0..2 {
                let [dr, dc] = directions[i];
                for _ in 0..steps {
                    if r >= 0 && c >= 0 && r < rows && c < cols {
                        res.push(vec![r, c]);
                    }
                    r += dr;
                    c += dc;
                }
                i = (i + 1) % 4;
            }
            steps += 1;
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
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::spiral_matrix_iii(3, 3, 0, 0),
            [
                [0, 0],
                [0, 1],
                [1, 1],
                [1, 0],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 1],
                [2, 0]
            ]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        )
    }
}
