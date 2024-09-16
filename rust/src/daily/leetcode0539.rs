// 539. Minimum Time Difference
pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points = time_points
            .into_iter()
            .map(|h| {
                let mut minutes = 0;
                let mut cur = 0;
                for c in h.chars() {
                    match c {
                        ':' => {
                            minutes = cur * 60;
                            cur = 0;
                        }
                        _ => {
                            cur *= 10;
                            cur += c.to_digit(10).unwrap() as i32;
                        }
                    }
                }
                minutes + cur
            })
            .collect::<Vec<i32>>();
        time_points.sort_unstable();
        let mut res = (1440 - time_points.last().unwrap()) + time_points[0];
        for i in 1..time_points.len() {
            res = res.min(time_points[i] - time_points[i - 1]);
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
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
            1
        );
    }
}
