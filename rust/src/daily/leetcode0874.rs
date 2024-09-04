// 874. Walking Robot Simulation
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // north = 0, east = 1, south = 2, west = 3
        let mut robot_dir: i8 = 0;
        let mut x_pos: i32 = 0;
        let mut y_pos: i32 = 0;
        let mut res = 0;
        let obstacles: HashSet<(i32, i32)> =
            HashSet::from_iter(obstacles.into_iter().map(|v| (v[0], v[1])));
        for cmd in commands {
            match cmd {
                -1 => robot_dir = (robot_dir + 1).rem_euclid(4),
                -2 => robot_dir = (robot_dir - 1).rem_euclid(4),
                _ => {
                    let (dir_x, dir_y) = match robot_dir {
                        0 => (0, 1),
                        1 => (1, 0),
                        2 => (0, -1),
                        3 => (-1, 0),
                        _ => panic!("invalid dir: {}", robot_dir),
                    };

                    for _ in 0..cmd {
                        if obstacles.contains(&(x_pos + dir_x, y_pos + dir_y)) {
                            break;
                        }
                        x_pos += dir_x;
                        y_pos += dir_y;
                    }
                    res = res.max(x_pos.pow(2) + y_pos.pow(2));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::robot_sim(vec![6, -1, -1, 6], vec![]), 36);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::robot_sim(
                vec![-2, 8, 3, 7, -1],
                vec![
                    vec![-4, -1],
                    vec![1, -1],
                    vec![1, 4],
                    vec![5, 0],
                    vec![4, 5],
                    vec![-2, -1],
                    vec![2, -5],
                    vec![5, 1],
                    vec![-3, -1],
                    vec![5, -3]
                ]
            ),
            324
        );
    }
}
