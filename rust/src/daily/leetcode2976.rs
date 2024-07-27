// 2976. Minimum Cost to Convert String I
pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut cache: [[i64; 26]; 26] = [[i64::MAX; 26]; 26];
        let mut map: HashMap<usize, Vec<(i64, usize)>> = HashMap::new();

        for ((from, to), cost) in original
            .into_iter()
            .map(|v| (v as u8 - b'a') as usize)
            .zip(changed.into_iter().map(|v| (v as u8 - b'a') as usize))
            .zip(cost.into_iter())
        {
            map.entry(from)
                .or_insert(Vec::new())
                .push((cost as i64, to));
        }

        fn cool_dijkstra(
            src: usize,
            cache: &mut [[i64; 26]; 26],
            map: &HashMap<usize, Vec<(i64, usize)>>,
        ) {
            let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
            heap.push((Reverse(0), src));
            cache[src][src] = 0;

            while let Some((Reverse(dist), to)) = heap.pop() {
                if let Some(val) = map.get(&to) {
                    for &(cost, destination) in val {
                        if cache[src][destination] <= dist + cost {
                            continue;
                        }
                        cache[src][destination] = dist + cost;
                        heap.push((Reverse(dist + cost), destination));
                    }
                }
            }
        }

        let mut res = 0_i64;
        for (src, tar) in source
            .as_bytes()
            .into_iter()
            .map(|v| (v - b'a') as usize)
            .zip(target.as_bytes().into_iter().map(|v| (v - b'a') as usize))
        {
            if cache[src][tar] == i64::MAX {
                cool_dijkstra(src, &mut cache, &map);
                return -1;
            }
            if cache[src][tar] == i64::MAX {
                return -1;
            }
            res += cache[src][tar];
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
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                //    2    5    5    1    2    20
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        )
    }
}
