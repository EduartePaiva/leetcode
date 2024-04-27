//use std::collections::VecDeque;

// 514. Freedom Trail
pub struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        let mut char_map: [Vec<usize>; 26] = Default::default();
        for i in 0..ring.len() {
            let index = (ring[i] - b'a') as usize;
            char_map[index].push(i);
        }
        let key: Vec<usize> = key
            .as_bytes()
            .into_iter()
            .map(|k| (k - b'a') as usize)
            .collect();

        let mut dp = vec![0; ring.len()];
        for k in (0..key.len()).rev() {
            let mut next_dp = vec![0; ring.len()];
            for r in 0..ring.len() {
                let mut min_cost = i32::MAX;
                let chars = &char_map[key[k]];
                for c in chars {
                    let min_dist =
                        i32::min(r.abs_diff(*c) as i32, (ring.len() - r.abs_diff(*c)) as i32);
                    min_cost = min_cost.min(dp[*c] + min_dist);
                }
                next_dp[r] = min_cost;
            }
            dp = next_dp;
        }
        return dp[0] + key.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
    }
}
