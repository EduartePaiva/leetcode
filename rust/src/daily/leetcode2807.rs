// 2807. Insert Greatest Common Divisors in Linked List

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn euclidean_algorithm(a: i32, b: i32) -> i32 {
            let (a, b) = if a >= b { (a, b) } else { (b, a) };
            if b == 0 {
                return a;
            }
            return euclidean_algorithm(b, a % b);
        }

        let mut dummy: Option<Box<ListNode>> = head;
        let mut head = dummy.as_mut().unwrap().next.take();
        let mut d_tail = &mut dummy;

        while let Some(mut cur) = head {
            let next_h = cur.next.take();
            head = next_h;

            // find the greatest common divisor between tail and cur
            if let Some(prev) = d_tail {
                let gcd = euclidean_algorithm(cur.val, prev.val);
                prev.next = Some(Box::new(ListNode {
                    val: gcd,
                    next: Some(cur),
                }));
                d_tail = &mut prev.next.as_mut().unwrap().next;
            }
        }

        dummy
    }
}

#[cfg(test)]
mod tests {
    fn gen_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        list.into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { val, next })))
    }

    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(gen_list(vec![18, 6, 10, 3])),
            gen_list(vec![18, 6, 6, 2, 10, 1, 3])
        );
    }
}
