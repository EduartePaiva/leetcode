// 1861. Rotating the Box
pub struct Solution;

impl Solution {
    pub fn rotate_the_box(caixa: Vec<Vec<char>>) -> Vec<Vec<char>> {
        // first rotate it clockwise
        let mut new_box = vec![vec!['.'; caixa.len()]; caixa[0].len()];

        // iterate the the matrix from the last column, to first column and last row to first row
        fn insert_stone(new_box: &mut Vec<Vec<char>>, mut row: usize, col: usize) {
            // the row will go the max it can down
            while row < new_box.len() && new_box[row][col] == '.' {
                row += 1;
            }
            new_box[row - 1][col] = '#';
        }
        let nb_cols = new_box[0].len() - 1;
        for c in (0..caixa[0].len()).rev() {
            for r in (0..caixa.len()).rev() {
                if caixa[r][c] == '#' {
                    insert_stone(&mut new_box, c, nb_cols - r);
                } else {
                    new_box[c][nb_cols - r] = caixa[r][c];
                }
            }
        }
        new_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::rotate_the_box(vec![vec!['#', '.', '#']]),
            vec![vec!['.'], vec!['#'], vec!['#']]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']]),
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }
}
