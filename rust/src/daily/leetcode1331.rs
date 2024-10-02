// 1331. Rank Transform of an Array
pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<_> = arr.into_iter().enumerate().collect();
        arr.sort_unstable_by_key(|x| x.1);

        let mut res: Vec<i32> = Vec::with_capacity(arr.len());
        let mut rank = 1;
        res.push(rank);
        for i in 1..arr.len() {
            if arr[i].1 != arr[i - 1].1 {
                rank += 1;
            }
            res.push(rank);
        }
        let mut arr: Vec<_> = arr.into_iter().zip(res.into_iter()).collect();
        arr.sort_by_cached_key(|x| x.0 .0);

        arr.into_iter().map(|x| x.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }
}
