// 2326. Spiral Matrix IV
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (m as usize, n as usize);
        let mut res = vec![vec![-1; cols]; rows];
        let (mut r_s, mut r_e, mut c_s, mut c_e) = (0, rows, 0, cols);
        while head.is_some() {
            // left to right
            for c in c_s..c_e {
                if let Some(h) = head {
                    res[r_s][c] = h.val;
                    head = h.next;
                }
            }
            r_s += 1;
            if r_e - r_s == 0 {
                break;
            }
            // top to down
            for r in r_s..r_e {
                if let Some(h) = head {
                    res[r][c_e - 1] = h.val;
                    head = h.next;
                }
            }
            c_e -= 1;
            if c_e - c_s == 0 {
                break;
            }
            // right to left
            for c in (c_s..c_e).rev() {
                if let Some(h) = head {
                    res[r_e - 1][c] = h.val;
                    head = h.next;
                }
            }
            r_e -= 1;
            if r_e - r_s == 0 {
                break;
            }
            // down to top
            for r in (r_s..r_e).rev() {
                if let Some(h) = head {
                    res[r][c_s] = h.val;
                    head = h.next;
                }
            }
            c_s += 1;
            if c_e - c_s == 0 {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        list.into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::spiral_matrix(
                3,
                5,
                build_list(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])
            ),
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ]
        )
    }
}
