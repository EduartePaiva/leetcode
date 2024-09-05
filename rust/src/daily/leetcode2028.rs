// 2028. Find Missing Observations
pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mean = mean as f64;
        let mut min_ans = n;
        let mut max_ans = n * 6;
        let total_dices = n + rolls.len() as i32;
        let rolls_sum: i32 = rolls.into_iter().sum();
        while min_ans < max_ans {
            let half = (max_ans - min_ans) / 2 + min_ans;
            let cur_mean = (half + rolls_sum) as f64 / total_dices as f64;
            if cur_mean < mean {
                min_ans = half + 1;
            } else if cur_mean > mean {
                max_ans = half - 1;
            } else {
                min_ans = half;
                break;
            }
        }
        let cur_mean = (min_ans + rolls_sum) as f64 / total_dices as f64;
        if cur_mean != mean {
            return vec![];
        }
        let mut res = vec![1; n as usize];
        min_ans -= n;
        for i in 0..res.len() {
            if min_ans >= 5 {
                res[i] = 6;
                min_ans -= 5;
            } else {
                res[i] += min_ans;
                break;
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6])
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4),
            vec![6, 1, 1, 1]
        )
    }
}
