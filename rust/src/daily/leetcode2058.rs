// 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        // turn this list in a normal list
        let mut list = vec![];
        while let Some(mut n) = head {
            list.push(n.val);
            head = n.next.take();
        }
        let mut critical_map = vec![false; list.len()];
        for i in 1..list.len() - 1 {
            if list[i] < list[i - 1] && list[i] < list[i + 1] {
                critical_map[i] = true;
            }
            if list[i] > list[i - 1] && list[i] > list[i + 1] {
                critical_map[i] = true;
            }
        }
        let mut gap_stack = vec![];
        for i in 0..critical_map.len() {
            if critical_map[i] {
                gap_stack.push(i);
            }
        }
        if gap_stack.len() < 2 {
            return vec![-1, -1];
        }
        let mut min_gap = i32::MAX;
        for i in 1..gap_stack.len() {
            min_gap = min_gap.min((gap_stack[i] - gap_stack[i - 1]) as i32);
        }
        vec![min_gap, (gap_stack.last().unwrap() - gap_stack[0]) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::nodes_between_critical_points(Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(1)))
            }))),
            vec![-1, -1]
        );
    }
}
