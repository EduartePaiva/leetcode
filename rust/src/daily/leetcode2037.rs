// 2037. Minimum Number of Moves to Seat Everyone
pub struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut students_map = HashMap::new();
        for stud in students {
            *students_map.entry(stud).or_insert(0) += 1;
        }
        let mut available_seats = vec![];
        for seat in seats {
            if students_map.contains_key(&seat) {
                let entry = students_map.entry(seat).or_insert(0);
                *entry -= 1;
                if entry == &0 {
                    students_map.remove(&seat);
                }
            } else {
                available_seats.push(seat);
            }
        }
        let mut available_students = vec![];
        for (key, val) in students_map.into_iter() {
            available_students.extend((0..val).map(|_| key));
        }
        let mut available_seats = BinaryHeap::from(available_seats);
        let mut available_students = BinaryHeap::from(available_students);

        let mut res = 0;
        while !available_seats.is_empty() {
            res += (available_seats
                .pop()
                .unwrap()
                .abs_diff(available_students.pop().unwrap())) as i32;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }
}
