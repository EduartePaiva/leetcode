// 2181. Merge Nodes in Between Zeros

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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut summed_nodes: Vec<i32> = vec![];
        let mut head = &head.unwrap().next;

        let mut sum = 0;
        while let Some(node) = &head {
            if node.val == 0 {
                summed_nodes.push(sum);
                sum = 0;
            } else {
                sum += node.val;
            }
            head = &node.next;
        }

        summed_nodes
            .into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { val, next })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        // [0,3,1,0,4,5,2,0]
        let head = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 2,
                                    next: Some(Box::new(ListNode::new(0))),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let res = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(11))),
        }));

        assert_eq!(Solution::merge_nodes(head), res);
    }
}
