// 796. Rotate String
pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let s = s.as_bytes();
        let goal = goal.as_bytes();

        fn check_equality(s: &[u8], goal: &[u8], s_start: usize) -> bool {
            let mut goal_index = 0;
            for i in s_start..s.len() {
                if s[i] != goal[goal_index] {
                    return false;
                }
                goal_index += 1;
            }
            for i in 0..s_start {
                if s[i] != goal[goal_index] {
                    return false;
                }
                goal_index += 1;
            }

            true
        }
        for i in 0..s.len() {
            if check_equality(s, goal, i) {
                return true;
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
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
    }
}
