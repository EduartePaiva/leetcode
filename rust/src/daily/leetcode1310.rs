// 1310. XOR Queries of a Subarray
pub struct Solution;

impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..arr.len() {
            arr[i] = arr[i - 1] ^ arr[i];
        }
        for query in queries {
            let start = query[0] as usize;
            let end = query[1] as usize;
            let xor = if start > 0 {
                arr[start - 1] ^ arr[end]
            } else {
                arr[end]
            };
            res.push(xor);
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
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            ),
            vec![2, 7, 14, 8]
        )
    }
}
