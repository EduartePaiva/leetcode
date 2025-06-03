use std::collections::VecDeque;

// 1298. Maximum Candies You Can Get from Boxes
pub struct Solution;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        mut candies: Vec<i32>,
        mut keys: Vec<Vec<i32>>,
        mut contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut total_candies = 0;

        let mut my_boxes: VecDeque<i32> = VecDeque::from(initial_boxes);

        let mut unlocked_box = true;
        while unlocked_box {
            unlocked_box = false;
            for _ in 0..my_boxes.len() {
                let cur_box = my_boxes.pop_front().unwrap() as usize;
                if status[cur_box] == 0 {
                    my_boxes.push_back(cur_box as i32);
                    continue;
                }

                while !keys[cur_box].is_empty() {
                    unlocked_box = true;
                    status[keys[cur_box].pop().unwrap() as usize] = 1;
                }

                while !contained_boxes[cur_box].is_empty() {
                    unlocked_box = true;

                    my_boxes.push_back(contained_boxes[cur_box].pop().unwrap());
                }

                total_candies += candies[cur_box];
                candies[cur_box] = 0;
            }
        }

        total_candies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0]
            ),
            16
        )
    }
}
