// 234. Palindrome Linked List
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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut list = vec![];

        loop {
            match head {
                Some(node) => {
                    list.push(node.val);
                    head = node.next
                }
                None => break,
            }
        }

        let mut right = list.len() - 1;
        let mut left = 0;

        while right > left {
            if list[right] != list[left] {
                return false;
            }
            right -= 1;
            left += 1;
        }
        true
    }
}

pub fn get_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    v.into_iter()
        .rev()
        .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(get_list(vec![1, 2, 2, 1])), true);
    }
}
