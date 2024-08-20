// 1140. Stone Game II
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut cache: HashMap<(bool, usize, usize), i32> = HashMap::new();

        fn dfs(
            cache: &mut HashMap<(bool, usize, usize), i32>,
            piles: &Vec<i32>,
            alice: bool,
            i: usize,
            m: usize,
        ) -> i32 {
            if i == piles.len() {
                return 0;
            }
            if let Some(v) = cache.get(&(alice, i, m)) {
                return *v;
            }
            let mut res = if alice { 0 } else { i32::MAX };

            let mut score_taken = 0;
            for x in i..piles.len().min(i + m * 2) {
                score_taken += piles[x];
                if alice {
                    res = res.max(score_taken + dfs(cache, piles, false, x + 1, m.max(x + 1 - i)));
                } else {
                    res = res.min(dfs(cache, piles, true, x + 1, m.max(x + 1 - i)));
                }
            }
            cache.insert((alice, i, m), res);
            res
        }

        dfs(&mut cache, &piles, true, 0, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10)
    }
}
