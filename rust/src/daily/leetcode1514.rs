// 1514. Path with Maximum Probability
pub struct Solution;

#[derive(PartialEq, PartialOrd)]
struct FloatOrd(f64);
impl Eq for FloatOrd {}
impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn max_probability(
        _n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut adj: HashMap<i32, Vec<(f64, i32)>> = HashMap::new();
        let mut visit: HashMap<i32, f64> = HashMap::new();

        for (edge, prob) in edges.into_iter().zip(succ_prob.into_iter()) {
            adj.entry(edge[0])
                .or_insert(Vec::new())
                .push((prob, edge[1]));
            adj.entry(edge[1])
                .or_insert(Vec::new())
                .push((prob, edge[0]));
        }

        let mut heap: BinaryHeap<(FloatOrd, i32)> = BinaryHeap::new();
        heap.push((FloatOrd(1.0), start_node));

        while let Some((FloatOrd(prob), node)) = heap.pop() {
            if node == end_node {
                return prob;
            }
            if let Some(neighbors) = adj.get(&node) {
                for &(p, n) in neighbors {
                    let cur_p = p * prob;
                    if let Some(&visit_p) = visit.get(&n) {
                        if visit_p >= cur_p {
                            continue;
                        }
                    }
                    visit.insert(n, cur_p);
                    heap.push((FloatOrd(cur_p), n));
                }
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2
            ),
            0.25
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_probability(
                500,
                vec![vec![193, 229], vec![133, 212], vec![224, 465]],
                vec![0.91, 0.78, 0.64],
                4,
                364
            ),
            0.0
        )
    }
}
