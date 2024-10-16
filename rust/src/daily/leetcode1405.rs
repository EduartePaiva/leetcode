// 1405. Longest Happy String
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        let mut res: Vec<char> = Vec::new();
        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }
        while !heap.is_empty() {
            let (n, c) = heap.pop().unwrap();
            if res.len() >= 2 && res[res.len() - 1] == c && res[res.len() - 2] == c {
                if heap.is_empty() {
                    break;
                }
                let (n2, c2) = heap.pop().unwrap();
                res.push(c2);
                heap.push((n, c));
                if n2 > 1 {
                    heap.push((n2 - 1, c2));
                }
                continue;
            }
            res.push(c);
            if n > 1 {
                heap.push((n - 1, c));
            }
        }
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            "ccbccacc".to_string()
        );
    }
}
