// 2285. Maximum Total Importance of Roads
pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut count = vec![0_i64; n as usize];
        let mut n = n as i64;
        for n in &roads {
            count[n[0] as usize] += 1;
            count[n[1] as usize] += 1;
        }

        let mut res = 0;
        count.sort_unstable();
        for cnt in count.into_iter().rev() {
            res += cnt * n;
            n -= 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_importance(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4]
                ]
            ),
            43
        );
    }
}
