//2487. Remove Nodes From Linked List

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
impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // monotonic stack
        let mut stack: Vec<Box<ListNode>> = vec![];
        while let Some(mut val) = head {
            while !stack.is_empty() && stack.last().unwrap().val < val.val {
                stack.pop();
            }
            head = val.next.take();
            stack.push(val)
        }

        stack.into_iter().rev().fold(None, |prev, mut cur| {
            cur.next = prev;
            Some(cur)
        })
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test1() {}
}
