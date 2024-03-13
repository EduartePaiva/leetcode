// 1171. Remove Zero Sum Consecutive Nodes from Linked List
pub struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn generate_list(nodes: Vec<i32>) -> Option<Box<ListNode>> {
    nodes
        .into_iter()
        .rev()
        .fold(None, |next, val| Some(Box::new(ListNode { val, next })))
}

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        while let Some(n) = head {
            head = n.next;
        }

        generate_list(vec![3, 1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_zero_sum_sublists(generate_list(vec![1, 2, -3, 3, 1])),
            generate_list(vec![3, 1])
        );
    }
}
