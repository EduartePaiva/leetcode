// 1497. Check If Array Pairs Are Divisible by k

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n.rem_euclid(k)).or_insert(0) += 1;
        }

        while !map.is_empty() {
            let (&key, &qtd) = map.iter().next().unwrap().clone();
            map.remove(&key);
            let other_key = (k - key) % k;
            if other_key == key && qtd % 2 == 0 {
                continue;
            }
            if let Some(&new_qtd) = map.get(&other_key) {
                if qtd != new_qtd {
                    return false;
                }
                map.remove(&other_key);
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5),
            true
        )
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true)
    }
}
