// 725. Split Linked List in Parts

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
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut list_size = 0;
        let mut temp = &head;
        while let Some(list) = temp {
            list_size += 1;
            temp = &list.next;
        }
        let base_size = list_size / k;
        let mut remaining = list_size % k;
        let mut res = vec![];

        while head.is_some() {
            let mut first_point = &mut head;
            let step = if remaining > 0 {
                remaining -= 1;
                base_size + 1
            } else {
                base_size
            };
            for _ in 1..step {
                if let Some(l) = first_point {
                    first_point = &mut l.next;
                } else {
                    break;
                }
            }
            let next_head = if let Some(l) = first_point {
                l.next.take()
            } else {
                None
            };
            res.push(head);
            head = next_head;
        }
        for _ in 0..k as usize - res.len() {
            res.push(None);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(
            Solution::split_list_to_parts(head, 5),
            vec![
                Some(Box::new(ListNode::new(1))),
                Some(Box::new(ListNode::new(2))),
                Some(Box::new(ListNode::new(3))),
                None,
                None
            ]
        );
    }
}
