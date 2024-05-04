// 881. Boats to Save People

pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut most_light = 0;
        let mut most_heavy = people.len() - 1;
        let mut min_num = 0;

        while most_light <= most_heavy && most_heavy < people.len() {
            let sum_boat = people[most_heavy] + people[most_light];
            min_num += 1;
            if sum_boat > limit {
                most_heavy -= 1;
            } else {
                most_heavy -= 1;
                most_light += 1;
            }
        }

        min_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    }
}
