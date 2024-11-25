// 773. Sliding Puzzle
pub struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mapping: [Vec<usize>; 6] = [
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];
        let current = [
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
        ];
        let to_find = [1, 2, 3, 4, 5, 0];
        let mut visit: HashSet<[i32; 6]> = HashSet::new();
        let mut deq = VecDeque::new();
        deq.push_back(current);
        visit.insert(current);

        let mut steps = 0;
        while !deq.is_empty() {
            for _ in 0..deq.len() {
                let brd = deq.pop_front().unwrap();
                if brd == to_find {
                    return steps;
                }
                let idx = brd.iter().position(|x| *x == 0).unwrap();
                for &change in &mapping[idx] {
                    let mut new_brd = brd.clone();
                    new_brd[idx] = new_brd[change];
                    new_brd[change] = 0;
                    if !visit.contains(&new_brd) {
                        deq.push_back(new_brd);
                        visit.insert(new_brd);
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
            1
        )
    }
}
