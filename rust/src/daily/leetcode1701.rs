// 1701. Average Waiting Time
pub struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut chef_time = 0;
        let mut time_sum = 0_f64;
        let total_customers = customers.len() as f64;

        //customers[i] = [arrival i, time i]
        for customer in customers {
            chef_time = chef_time.max(customer[0]) + customer[1];
            time_sum += (chef_time - customer[0]) as f64;
        }

        time_sum / total_customers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.0
        );
    }
}
