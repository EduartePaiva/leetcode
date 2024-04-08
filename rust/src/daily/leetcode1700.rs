// 1700. Number of Students Unable to Eat Lunch

pub struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut square_sand = 0;
        let mut circular_sand = 0;

        for n in students {
            if n == 0 {
                circular_sand += 1;
            } else {
                square_sand += 1;
            }
        }

        for sand in sandwiches {
            if sand == 0 {
                if circular_sand > 0 {
                    circular_sand -= 1;
                } else {
                    return square_sand;
                }
            } else {
                if square_sand > 0 {
                    square_sand -= 1;
                } else {
                    return circular_sand;
                }
            }
        }

        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
