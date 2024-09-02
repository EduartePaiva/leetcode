// 1894. Find the Student that Will Replace the Chalk
pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|x| *x as i64).sum();
        k = (k as i64 % sum) as i32;
        for i in 0..chalk.len() {
            if k < chalk[i] {
                return i as i32;
            }
            k -= chalk[i];
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0)
    }
}
