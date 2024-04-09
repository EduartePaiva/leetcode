// 2073. Time Needed to Buy Tickets
pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k_val = tickets[k as usize];

        for i in 0..=(k as usize) {
            res += tickets[i].min(k_val);
        }
        for i in ((k + 1) as usize)..tickets.len() {
            res += tickets[i].min(k_val - 1);
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::time_required_to_buy(vec![84, 49, 5, 24, 70, 77, 87, 8], 3),
            154
        );
    }
}
