// 1823. Find the Winner of the Circular Game
pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut res = 0;
        // k = 2, people = 9 = 2
        // k = 2, people = 8 = 0
        // k = 2, people = 7 = 6
        // k = 2, people = 6 = 4
        // k = 2, people = 5 = 2
        // k = 2, people = 4 = 0
        // k = 2, people = 3 = 2
        // k = 2, people = 2 = 0
        // k = 2, people = 1 = 0

        for people in 2..=n {
            res = (res + k) % people;
        }
        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_the_winner(9, 2), 3);
    }
}
