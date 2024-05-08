// 506. Relative Ranks
pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score = score
            .into_iter()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect::<Vec<_>>();
        score.sort_unstable();
        score.reverse();

        let mut res: Vec<String> = (1..=score.len()).map(|i| i.to_string()).collect();
        if res.len() > 0 {
            res[0] = "Gold Medal".to_string();
        }
        if res.len() > 1 {
            res[1] = "Silver Medal".to_string();
        }
        if res.len() > 2 {
            res[2] = "Bronze Medal".to_string();
        }

        let mut res = score
            .into_iter()
            .map(|v| v.1)
            .zip(res.into_iter())
            .collect::<Vec<_>>();
        res.sort_unstable();

        res.into_iter().map(|v| v.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
    }
}
