// 135. Candy
pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut distribution = vec![1_i32; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                distribution[i] = distribution[i - 1] + 1;
            }
            println!("{:?}", distribution);
        }
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                distribution[i] = distribution[i].max(distribution[i + 1] + 1);
            }
            println!("{:?}", distribution);
        }

        distribution.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::candy(vec![1, 3, 4, 5, 2]), 11)

        // 1,3,4,5,2
        // 1,2,3,4,1
    }
}
