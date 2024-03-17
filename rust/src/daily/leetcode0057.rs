// 57. Insert Interval

pub struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        res.push(new_interval);

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];
            let lst_index = res.len() - 1;

            //case where don't overlap
            if res[lst_index][1] < start {
                res.push(interval);
                continue;
            }
            //case where the last is beyond the interval
            if end < res[lst_index][0] {
                res.insert(lst_index, interval);
                continue;
            }

            //case where overlap
            res[lst_index][0] = std::cmp::min(res[lst_index][0], start);
            res[lst_index][1] = std::cmp::max(res[lst_index][1], end);
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5],),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8],
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
