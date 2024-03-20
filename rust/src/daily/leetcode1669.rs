// 1669. Merge In Between Linked Lists

pub struct Solution;

//Definition for singly-linked list.
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
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let a = a as usize;
        let b = (b + 1) as usize;
        let mut list_vec1 = vec![];
        let mut list_vec2 = vec![];

        while let Some(val) = list1 {
            list_vec1.push(val.val);
            list1 = val.next;
        }
        while let Some(val) = list2 {
            list_vec2.push(val.val);
            list2 = val.next;
        }
        let _: Vec<_> = list_vec1.splice(a..b, list_vec2).collect();
        list_vec1
            .into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
    }
}
pub fn create_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    list.into_iter()
        .rev()
        .fold(None, |next, val| Some(Box::new(ListNode { next, val })))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_in_between(
                create_list(vec![10, 1, 13, 6, 9, 5]),
                3,
                4,
                create_list(vec![1000000, 1000001, 1000002])
            ),
            create_list(vec![10, 1, 13, 1000000, 1000001, 1000002, 5])
        )
    }
}
