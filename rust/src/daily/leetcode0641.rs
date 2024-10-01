// 641. Design Circular Deque

#![allow(unused)]
use std::collections::VecDeque;
struct MyCircularDeque {
    deq: VecDeque<i32>,
    max_len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            deq: VecDeque::new(),
            max_len: k as usize,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.deq.len() == self.max_len {
            return false;
        }
        self.deq.push_front(value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.deq.len() == self.max_len {
            return false;
        }
        self.deq.push_back(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        self.deq.pop_front().is_some()
    }

    fn delete_last(&mut self) -> bool {
        self.deq.pop_back().is_some()
    }

    fn get_front(&self) -> i32 {
        *self.deq.front().unwrap_or(&-1)
    }

    fn get_rear(&self) -> i32 {
        *self.deq.back().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }

    fn is_full(&self) -> bool {
        self.deq.len() == self.max_len
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut my_circ = MyCircularDeque::new(3);
        assert_eq!(my_circ.insert_last(1), true);
        assert_eq!(my_circ.insert_last(2), true);
        assert_eq!(my_circ.insert_front(3), true);
        assert_eq!(my_circ.insert_front(4), false);
        assert_eq!(my_circ.get_rear(), 2);
        assert_eq!(my_circ.is_full(), true);
        assert_eq!(my_circ.delete_last(), true);
        assert_eq!(my_circ.insert_front(4), true);
        assert_eq!(my_circ.get_front(), 4);
    }
}
