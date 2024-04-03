//79. Word Search

pub struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;

        fn dfs(
            row: i32,
            col: i32,
            word_index: usize,
            board: &mut Vec<Vec<char>>,
            word: &Vec<char>,
        ) -> bool {
            if word_index == word.len() {
                return true;
            }
            if row < 0 || col < 0 {
                return false;
            }
            let row_u = row as usize;
            let col_u = col as usize;

            if row_u == board.len()
                || col_u == board[0].len()
                || word[word_index] != board[row_u][col_u]
            {
                return false;
            }
            let char = board[row_u][col_u];
            board[row_u][col_u] = ' ';
            let val = dfs(row + 1, col, word_index + 1, board, word)
                || dfs(row - 1, col, word_index + 1, board, word)
                || dfs(row, col + 1, word_index + 1, board, word)
                || dfs(row, col - 1, word_index + 1, board, word);
            board[row_u][col_u] = char;

            return val;
        }

        for row in 0..rows {
            for col in 0..cols {
                if dfs(row, col, 0, &mut board, &word) {
                    return true;
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(Solution::exist(board, "ABCCED".to_string()), true)
    }

    #[test]
    fn test2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(Solution::exist(board, "ABCESEEEFS".to_string()), true)
    }
}
