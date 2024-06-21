// 1052. Grumpy Bookstore Owner
pub struct Solution;
// customers
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;
        let mut most_satisfied = 0;
        let mut cur_satisfied = 0;
        for i in 0..customers.len() {
            if i >= minutes && grumpy[i - minutes] == 1 {
                cur_satisfied -= customers[i - minutes];
            }
            if grumpy[i] == 1 {
                cur_satisfied += customers[i];
            }
            most_satisfied = most_satisfied.max(cur_satisfied);
        }

        let mut total_customers = 0;
        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                total_customers += customers[i];
            }
        }

        total_customers + most_satisfied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_satisfied(vec![9, 10, 4, 5], vec![1, 0, 1, 1], 1),
            19
        );
    }
}
