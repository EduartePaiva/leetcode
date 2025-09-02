// 3025. Find the Number of Ways to Place People I
pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        // Step 1: Sort points by x ascending, then by y descending
        points.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut pair_count = 0;
        let n = points.len();

        // Step 2: Iterate through all points as potential "upper-left" points
        for i in 0..n {
            let upper_y = points[i][1];
            let mut lower_y_limit = i32::MIN;
            // Step 3: Check subsequent points as potential "bottom-right" points
            for j in (i + 1)..n {
                let current_y = points[j][1];
                if current_y <= upper_y && current_y > lower_y_limit {
                    pair_count += 1;
                    lower_y_limit = current_y;
                    if current_y == upper_y {
                        break;
                    }
                }
            }
        }
        pair_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![6, 2], vec![4, 4], vec![2, 6]]),
            2
        )
    }
}
