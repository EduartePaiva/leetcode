// 1395. Count Number of Teams
pub struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..rating.len() - 1 {
            let mid = rating[i];
            let mut cnt_less = 0;
            let mut cnt_greater = 0;
            for j in 0..i {
                if rating[j] < mid {
                    cnt_less += 1;
                }
            }
            for j in i + 1..rating.len() {
                if rating[j] > mid {
                    cnt_greater += 1;
                }
            }
            res += cnt_less * cnt_greater;
            cnt_less = i as i32 - cnt_less;
            cnt_greater = (rating.len() - i - 1 - cnt_greater as usize) as i32;
            res += cnt_less * cnt_greater;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3)
    }
}
