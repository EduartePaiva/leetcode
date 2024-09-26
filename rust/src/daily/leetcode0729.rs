// 729. My Calendar I

struct MyCalendar {
    booking: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            booking: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.booking {
            // [10,20] [15,20], [20,30]
            // [19,30] [13,32]

            // 10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35
            // ...........................-----------------------------------...............
            // .........-----------------------------------------------------------.........

            // s=19, e=30, start=13, end=32
            //    13  < 30      13 >= 19      32 > 19    32 <= 30
            if start < e && end > s {
                return false;
            }
        }

        self.booking.push((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut obj = MyCalendar::new();
        assert_eq!(obj.book(10, 20), true);
        assert_eq!(obj.book(15, 25), false);
        assert_eq!(obj.book(20, 30), true);
    }
}
