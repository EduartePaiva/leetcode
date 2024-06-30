// 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
pub struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // union find problem
        // start with edges of type 3.
        let mut array_to_union: Vec<usize> = (0..n as usize + 1).collect();
        let mut size = vec![1_usize; n as usize + 1];
        fn find(mut num: usize, array_to_union: &mut Vec<usize>) -> usize {
            while array_to_union[num] != num {
                array_to_union[num] = array_to_union[array_to_union[num]];
                num = array_to_union[num];
            }
            num
        }

        fn union(
            one: usize,
            two: usize,
            array_to_union: &mut Vec<usize>,
            size: &mut Vec<usize>,
        ) -> i32 {
            let one = find(one, array_to_union);
            let two = find(two, array_to_union);
            // now I union them
            if one == two {
                return 1;
            }
            if size[one] >= size[two] {
                size[one] += size[two];
                array_to_union[two] = one;
            } else {
                size[two] += size[one];
                array_to_union[one] = two;
            }
            return 0;
        }

        let mut res = 0;
        for edge in &edges {
            if edge[0] == 3 {
                res += union(
                    edge[1] as usize,
                    edge[2] as usize,
                    &mut array_to_union,
                    &mut size,
                );
            }
        }

        let mut alice_atu = array_to_union.clone();
        let mut alice_size = size.clone();
        for edge in &edges {
            if edge[0] == 1 {
                res += union(
                    edge[1] as usize,
                    edge[2] as usize,
                    &mut alice_atu,
                    &mut alice_size,
                );
            }
        }
        if alice_size
            .into_iter()
            .find(|&v| v == (n as usize))
            .is_none()
        {
            return -1;
        }
        for edge in &edges {
            if edge[0] == 2 {
                res += union(
                    edge[1] as usize,
                    edge[2] as usize,
                    &mut array_to_union,
                    &mut size,
                );
            }
        }
        if size.into_iter().find(|&v| v == (n as usize)).is_none() {
            return -1;
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
            Solution::max_num_edges_to_remove(
                4,
                vec![
                    vec![3, 1, 2],
                    vec![3, 2, 3],
                    vec![1, 1, 3],
                    vec![1, 2, 4],
                    vec![1, 1, 2],
                    vec![2, 3, 4]
                ]
            ),
            2
        );
    }
}
