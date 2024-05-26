// 552. Student Attendance Record II
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: i32 = i32::pow(10, 9) + 7;
        fn dfs(i: i32, a: bool, l: u8, cache: &mut HashMap<(i32, bool, u8), i32>) -> i32 {
            if i == 1 {
                if a && l > 1 {
                    return 1;
                } else if a && l < 2 {
                    return 2;
                } else if l > 1 {
                    return 2;
                } else {
                    return 3;
                }
            }
            if let Some(val) = cache.get(&(i, a, l)) {
                return *val;
            }

            let mut res = 0;
            // pick A
            if !a {
                res = dfs(i - 1, true, 0, cache) % MOD;
            }
            // Pick L
            if l < 2 {
                res += dfs(i - 1, a, l + 1, cache) % MOD;
                res %= MOD;
            }

            // pick P
            res += dfs(i - 1, a, 0, cache) % MOD;
            res %= MOD;

            cache.insert((i, a, l), res % MOD);
            res % MOD
        }

        let mut cache: HashMap<(i32, bool, u8), i32> = HashMap::new();
        dfs(n, false, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::check_record(2), 8);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::check_record(1), 3);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
