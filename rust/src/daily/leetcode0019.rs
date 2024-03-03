// 19. Remove Nth Node From End of List

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut right = dummy.clone();
        let mut left = dummy.as_mut();

        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let list_head = generate_list(5);
        Solution::remove_nth_from_end(list_head, 1);
    }

    fn generate_list(n: i32) -> Option<Box<ListNode>> {
        let mut list_head = Some(Box::new(ListNode::new(0)));

        let mut list_point = &mut list_head;
        for i in 1..n {
            if let Some(x) = list_point {
                x.next = Some(Box::new(ListNode::new(i)));
                list_point = &mut x.next;
            }
        }

        list_head
    }
}
