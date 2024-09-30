// 1381. Design a Stack With Increment Operation
struct CustomStack {
    stk: Vec<i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            stk: Vec::with_capacity(max_size as usize),
            capacity: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stk.len() == self.capacity {
            return;
        }
        self.stk.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.stk.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..self.stk.len().min(k as usize) {
            self.stk[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut stk = CustomStack::new(3); // Stack is Empty []
        stk.push(1); // stack becomes [1]
        stk.push(2); // stack becomes [1, 2]
        assert_eq!(stk.pop(), 2); // return 2 --> Return top of the stack 2, stack becomes [1]
        stk.push(2); // stack becomes [1, 2]
        stk.push(3); // stack becomes [1, 2, 3]
        stk.push(4); // stack still [1, 2, 3], Do not add another elements as size is 4
        stk.increment(5, 100); // stack becomes [101, 102, 103]
        stk.increment(2, 100); // stack becomes [201, 202, 103]
        stk.pop(); // return 103 --> Return top of the stack 103, stack becomes [201, 202]
        stk.pop(); // return 202 --> Return top of the stack 202, stack becomes [201]
        stk.pop(); // return 201 --> Return top of the stack 201, stack becomes []
        stk.pop(); // return -1 --> Stack is empty return -1.
    }
}
