// 2751. Robot Collisions
pub struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let directions = directions.as_bytes();
        // turn in a tuple of (position, origin_pos, health, direction)
        let mut fused_all: Vec<(i32, usize, i32, u8)> = positions
            .into_iter()
            .zip((0..healths.len()).zip(healths.into_iter().zip(directions.into_iter())))
            .map(|(pos, (first_pos, (health, dir)))| (pos, first_pos, health, *dir))
            .collect();
        fused_all.sort_unstable();
        let mut result_stack: Vec<(i32, usize, i32, u8)> = vec![];

        // how directions can happen:
        // prev can be ) or (
        // cur can be ) or (

        // if prev == L I can just append cur
        // if prev == R and cur == L I just append cur
        // if prev == R and cur == L cur will keep going until it get destroyed or reach L or first position
        // println!("{:?}", fused_all);

        for cur_robot in fused_all {
            if result_stack.len() == 0 {
                result_stack.push(cur_robot);
                continue;
            }

            if cur_robot.3 == b'R' {
                result_stack.push(cur_robot);
                continue;
            }
            if let Some(robot) = result_stack.last() {
                if robot.3 == b'L' {
                    result_stack.push(cur_robot);
                    continue;
                }
            }
            let mut cur_health = cur_robot.2;
            while cur_health > 0 && result_stack.len() > 0 && result_stack.last().unwrap().3 == b'R'
            {
                let last_robot = result_stack.pop().unwrap();
                if cur_health == last_robot.2 {
                    cur_health = 0;
                    break;
                }

                if cur_health < last_robot.2 {
                    result_stack.push((last_robot.0, last_robot.1, last_robot.2 - 1, last_robot.3));
                    cur_health = 0;
                    break;
                }

                cur_health -= 1;
            }
            if cur_health > 0 {
                result_stack.push((cur_robot.0, cur_robot.1, cur_health, cur_robot.3));
            }
        }
        result_stack.sort_unstable_by_key(|k| k.1);

        result_stack.into_iter().map(|v| v.2).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![5, 4, 3, 2, 1],
                vec![2, 17, 9, 15, 10],
                "RRRRR".to_string()
            ),
            vec![2, 17, 9, 15, 10]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![3, 5, 2, 6],
                vec![10, 10, 15, 12],
                "RLRL".to_string()
            ),
            vec![14]
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::survived_robots_healths(vec![2], vec![2], "L".to_string()),
            vec![2]
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::survived_robots_healths(vec![1, 40], vec![10, 11], "RL".to_string()),
            vec![10]
        );
    }
}
