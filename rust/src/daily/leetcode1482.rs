// 1482. Minimum Number of Days to Make m Bouquets
pub struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        //binary search in the answer
        let mut res = -1;

        let mut min_day = *bloom_day.iter().min().unwrap();
        let mut max_day = *bloom_day.iter().max().unwrap();

        while min_day <= max_day {
            let half = (max_day - min_day) / 2 + min_day;

            //check if with half we can make m bouquets of k adjacent flowers.
            let mut consecutive_flowers = 0;
            let mut bouquet_cnt = 0;
            for &day in &bloom_day {
                if day <= half {
                    consecutive_flowers += 1;
                    if consecutive_flowers == k {
                        consecutive_flowers = 0;
                        bouquet_cnt += 1;
                    }
                } else {
                    consecutive_flowers = 0;
                }
            }
            if bouquet_cnt >= m {
                res = half;
                max_day = half - 1;
            } else {
                min_day = half + 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12)
    }
}
