use super::leetcode0234::ListNode;

// 143. Reorder List
pub struct Solution;
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut nodes = vec![];
        let mut new_head = head.take();

        // Transfer all nodes into the nodes vec
        while let Some(mut node) = new_head {
            new_head = node.next.take();
            nodes.push(node);
        }

        let length = nodes.len();

        // here I split nodes in half.
        let other_half = nodes.split_off(length / 2);

        // Here I make 2 iterators iter1 is the first half,
        // iter2 is the second half in reverse order.
        let mut iter1 = nodes.into_iter();
        let mut iter2 = other_half.into_iter().rev();

        // I recreate nodes to store the the nodes rearranged
        let mut nodes = Vec::with_capacity(length);

        //this while rearrange the nodes.
        while iter1.len() != 0 || iter2.len() != 0 {
            if let Some(node) = iter1.next() {
                nodes.push(node);
            }
            if let Some(node) = iter2.next() {
                nodes.push(node);
            }
        }

        // this rebuild the list using the vec of node
        let nodes = nodes.into_iter().rev().fold(None, |prev, mut node| {
            node.next = prev;
            Some(node)
        });

        // this gives ownership of the nodes back to head
        head.replace(nodes.unwrap());
    }
}
#[cfg(test)]
mod tests {
    use crate::daily::leetcode0234::get_list;

    use super::*;
    #[test]
    fn test1() {
        let mut first_list = get_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut first_list);
        assert_eq!(first_list, get_list(vec![1, 4, 2, 3]));
    }
}
