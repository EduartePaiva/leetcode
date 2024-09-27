// 731. My Calendar II
struct MyCalendarTwo {
    booking: Vec<(i32, i32)>,
    overlapping: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            booking: Vec::new(),
            overlapping: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.overlapping {
            if start < e && end > s {
                return false;
            }
        }
        for &(s, e) in &self.booking {
            if start < e && end > s {
                self.overlapping.push((s.max(start), e.min(end)));
            }
        }
        self.booking.push((start, end));
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;

    fn test_calendar(calendar: &mut MyCalendarTwo, to_add: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = vec![];
        for i in 0..to_add.len() {
            result.push(calendar.book(to_add[i][0], to_add[i][1]));
        }
        result
    }
    #[test]
    fn test1() {
        let mut calendar = MyCalendarTwo::new();
        assert_eq!(
            test_calendar(
                &mut calendar,
                vec![
                    vec![10, 20],
                    vec![50, 60],
                    vec![10, 40],
                    vec![5, 15],
                    vec![5, 10],
                    vec![25, 55],
                ],
            ),
            vec![true, true, true, false, true, true]
        );
    }
}
