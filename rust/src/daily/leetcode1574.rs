// 1574. Shortest Subarray to be Removed to Make Array Sorted
pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut prefix_sorted = vec![false; arr.len()];
        let mut postfix_sorted = vec![false; arr.len()];
        prefix_sorted[0] = true;
        for i in 1..arr.len() {
            if arr[i] >= arr[i - 1] {
                prefix_sorted[i] = true;
            } else {
                break;
            }
        }
        postfix_sorted[arr.len() - 1] = true;
        for i in (0..arr.len() - 1).rev() {
            if arr[i] <= arr[i + 1] {
                postfix_sorted[i] = true;
            } else {
                break;
            }
        }
        if postfix_sorted[0] {
            return 0;
        }
        let mut min = 1;
        let mut max = arr.len() - 2;
        let mut last_worked = max + 1;

        while min <= max {
            let gap = (max - min) / 2 + min;
            let mut gap_works = false;
            if postfix_sorted[gap] || prefix_sorted[arr.len() - 1 - gap] {
                gap_works = true;
            } else {
                for i in 0..arr.len() - gap - 1 {
                    if prefix_sorted[i] && postfix_sorted[i + gap + 1] && arr[i] <= arr[i + gap + 1]
                    {
                        gap_works = true;
                        break;
                    }
                }
            }
            if gap_works {
                max = gap - 1;
                last_worked = gap;
            } else {
                min = gap + 1;
            }
        }
        last_worked as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3,]),
            0
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 0, 7, 8, 9]),
            2
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![
                10, 13, 17, 21, 15, 15, 9, 17, 22, 22, 13
            ]),
            7
        );
    }
}
