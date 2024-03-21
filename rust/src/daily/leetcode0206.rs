// 206. Reverse Linked List
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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while head.is_some() {
            let next_head = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = prev;
            prev = head;
            head = next_head;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        fn generate_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
            vals.into_iter()
                .rev()
                .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
        }
        assert_eq!(
            Solution::reverse_list(generate_list(vec![1, 2, 3, 4, 5])),
            generate_list(vec![5, 4, 3, 2, 1])
        );
    }
}
