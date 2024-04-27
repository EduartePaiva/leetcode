//use std::collections::VecDeque;

// 514. Freedom Trail
pub struct Solution;

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

        fn greedy_dist(my_pos: usize, chars: &Vec<usize>, ring_size: usize) -> (i32, usize) {
            let mut steps = i32::MAX;
            let mut new_pos = 0;

            for c in chars {
                if c == &my_pos {
                    return (0, my_pos);
                }
                //going front
                if c > &my_pos {
                    let n_steps = (c - &my_pos) as i32;
                    if n_steps < steps {
                        steps = n_steps;
                        new_pos = *c;
                    }
                } else {
                    let n_steps = ((ring_size - my_pos) + *c) as i32;
                    if n_steps < steps {
                        steps = n_steps;
                        new_pos = *c;
                    }
                }
                //going back
            }

            (steps, new_pos)
        }

        fn dfs(ring_i: usize, key_i: usize, char_map: &[Vec<usize>; 26], key: &Vec<usize>) -> i32 {
            0
        }

        let steps = key.len() as i32;
        for k in key {
            let chars = &char_map[k];

            for char in chars {
                // here I know the current position of the char based on the variation on the ring
                // now I calculate the min move and if it's positive or negative
                // check for positive move
            }
        }

        println!("{:?}", char_map);

        //let mut ring_dec: VecDeque<u8> = ring.as_bytes().into_iter().map(|x| *x).collect();

        // map all characters
        // find all characters
        // find closest one

        steps
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
