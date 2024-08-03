// 1460. Make Two Arrays Equal by Reversing Subarrays
pub struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut map = [0; 1001];

        for n in target {
            map[n as usize] += 1;
        }
        for n in arr {
            map[n as usize] -= 1;
        }
        for n in map {
            if n != 0 {
                return false;
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
            Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
            true
        );
    }
}
