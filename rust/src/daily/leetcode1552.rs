// 1552. Magnetic Force Between Two Balls
pub struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        // binary search again?????
        let mut min = 1;
        let mut max = *position.last().unwrap() - position[0];
        // println!("this is max: {max}");

        while min <= max {
            // place the balls
            let half = (max - min) / 2 + min;
            // println!("Half: {half}, min: {min}, max: {max}");

            let mut total_m = 1;
            let mut prev = position[0];
            for i in 1..position.len() {
                if (prev.abs_diff(position[i]) as i32) >= half {
                    total_m += 1;
                    prev = position[i];
                    if total_m == m {
                        break;
                    }
                }
            }

            if total_m >= m {
                min = half + 1;
            } else {
                max = half - 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
