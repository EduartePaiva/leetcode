// 2337. Move Pieces to Obtain a String
pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        if start.replace('_', "") != target.replace('_', "") {
            return false;
        }
        let target = target.as_bytes();
        let start = start.as_bytes();
        let mut j = 0;
        for i in 0..target.len() {
            if target[i] == b'L' {
                j = j.max(i);
                while j < start.len() && start[j] == b'_' {
                    j += 1;
                }
                if j == start.len() || start[j] != b'L' {
                    return false;
                }
                j += 1;
            }
        }
        let mut j = (target.len() - 1) as isize;
        for i in (0..target.len()).rev() {
            if target[i] == b'R' {
                j = j.min(i as isize);
                while j >= 0 && start[j as usize] == b'_' {
                    j -= 1;
                }
                if j < 0 || start[j as usize] != b'R' {
                    return false;
                }
                j -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string()),
            true
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_change("R_L_".to_string(), "__LR".to_string()),
            false
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::can_change("R__L".to_string(), "_LR_".to_string()),
            false
        );
    }
}
