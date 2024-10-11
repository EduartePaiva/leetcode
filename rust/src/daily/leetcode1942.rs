// 1942. The Number of the Smallest Unoccupied Chair
pub struct Solution;

struct Friend {
    friend_id: usize,
    arrival: i32,
    leaving: i32,
}

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let target_friend = target_friend as usize;
        let mut friends = vec![];
        for friend_id in 0..times.len() {
            let arrival = times[friend_id][0];
            let leaving = times[friend_id][1];
            friends.push(Friend {
                arrival,
                leaving,
                friend_id,
            });
        }
        friends.sort_unstable_by_key(|f| f.arrival);
        let mut available_chairs: BinaryHeap<Reverse<usize>> =
            BinaryHeap::from_iter((0..friends.len()).map(|v| Reverse(v)));
        let mut unavailable_chairs: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        for Friend {
            friend_id,
            arrival,
            leaving,
        } in friends
        {
            while !unavailable_chairs.is_empty()
                && unavailable_chairs.peek().unwrap().0 .0 <= arrival
            {
                available_chairs.push(Reverse(unavailable_chairs.pop().unwrap().0 .1));
            }
            let avb_chair = available_chairs.pop().unwrap().0;
            if friend_id == target_friend {
                return avb_chair as i32;
            }
            unavailable_chairs.push(Reverse((leaving, avb_chair)));
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1),
            1
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0),
            2
        );
    }
}
