// 2924. Find Champion II
pub struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut nodes = vec![true; n as usize];
        for edge in edges {
            nodes[edge[1] as usize] = false;
        }
        let res: Vec<_> = nodes
            .into_iter()
            .enumerate()
            .filter_map(|x| if x.1 { Some(x.0 as i32) } else { None })
            .collect();
        if res.len() == 1 {
            res[0]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0)
    }
}
