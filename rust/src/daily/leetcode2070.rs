// 2070. Most Beautiful Item for Each Query
pub struct Solution;

struct Query {
    index: usize,
    query: i32,
    result: i32,
}

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|k| k[0]);
        items.reverse();
        let mut queries: Vec<Query> = queries
            .into_iter()
            .enumerate()
            .map(|v| Query {
                index: v.0,
                query: v.1,
                result: 0,
            })
            .collect();
        queries.sort_unstable_by_key(|k| k.query);

        let mut max_beauty = 0;
        for i in 0..queries.len() {
            while !items.is_empty() && items.last().unwrap()[0] <= queries[i].query {
                max_beauty = max_beauty.max(items.pop().unwrap()[1]);
            }
            queries[i].result = max_beauty;
        }
        queries.sort_unstable_by_key(|k| k.index);
        queries.into_iter().map(|k| k.result).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            ),
            vec![2, 4, 5, 5, 6, 6]
        );
    }
}
