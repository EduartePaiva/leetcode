// 1346. Check If N and Its Double Exist
pub struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for i in 0..arr.len() {
            for j in 0..arr.len() {
                if i == j {
                    continue;
                }
                if arr[i] == arr[j] * 2 {
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
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
    }
}
