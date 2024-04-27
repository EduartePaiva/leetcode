//use std::collections::VecDeque;

// 514. Freedom Trail
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        let mut char_map: [Vec<usize>; 26] = Default::default();
        for i in 0..ring.len() {
            let index = (ring[i] - b'a') as usize;
            char_map[index].push(i);
        }

        let key: Vec<usize> = key
            .as_bytes()
            .into_iter()
            .map(|k| (k - b'a') as usize)
            .collect();

        let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
        fn dfs(
            ring_i: usize,
            key_i: usize,
            char_map: &[Vec<usize>; 26],
            key: &Vec<usize>,
            ring_size: usize,
            cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if key.len() == key_i {
                return 0;
            }
            if let Some(val) = cache.get(&(ring_i, key_i)) {
                return *val;
            }
            let mut min_cost = i32::MAX;
            let chars = &char_map[key[key_i]];
            for c in chars {
                min_cost = min_cost.min(
                    dfs(*c, key_i + 1, char_map, key, ring_size, cache)
                        + i32::min(
                            key_i.abs_diff(*c) as i32,
                            (ring_size - key_i.abs_diff(*c)) as i32,
                        ),
                );
            }
            cache.insert((ring_i, key_i), min_cost);
            min_cost
        }

        dfs(0, 0, &char_map, &key, ring.len(), &mut cache) + key.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
    }
}
