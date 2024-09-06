// 3217. Delete Nodes From Linked List Present in Array
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

use std::collections::HashSet;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let mut res = vec![];
        while let Some(h) = head {
            if !nums.contains(&h.val) {
                res.push(h.val);
            }
            head = h.next;
        }

        res.into_iter().rev().fold(None, |prev, cur| {
            Some(Box::new(ListNode {
                val: cur,
                next: prev,
            }))
        })
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
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let res = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(5))),
        }));
        assert_eq!(Solution::modified_list(vec![1, 2, 3], head), res);
    }
}
