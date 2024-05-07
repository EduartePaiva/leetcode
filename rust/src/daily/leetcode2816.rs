// 2816. Double a Number Represented as a Linked List
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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            let mut cur = head;

            while let Some(mut node) = cur {
                let temp = node.next;
                node.next = prev;
                prev = Some(node);
                cur = temp;
            }

            prev
        }

        let mut new_head = reverse_list(head);
        let mut cur = &mut new_head;
        let mut carry = 0;

        while let Some(node) = cur {
            let val = node.val;
            node.val = (carry + val * 2) % 10;
            carry = (carry + val * 2) / 10;
            cur = &mut node.next;
        }

        if carry > 0 {
            *cur = Some(Box::new(ListNode::new(carry)));
        }

        reverse_list(new_head)
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
                val: 8,
                next: Some(Box::new(ListNode::new(9))),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        assert_eq!(Solution::double_it(head), result);
    }
    #[test]
    fn test2() {
        let head = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode::new(9))),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
        }));

        assert_eq!(Solution::double_it(head), result);
    }
}
