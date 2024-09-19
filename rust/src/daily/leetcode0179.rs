// 179. Largest Number
pub struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        if nums.iter().max().unwrap() == &0 {
            return "0".to_string();
        }

        nums.sort_by(|a, b| {
            String::from_iter([a.to_string(), b.to_string()])
                .parse::<i128>()
                .unwrap()
                .cmp(
                    &String::from_iter([b.to_string(), a.to_string()])
                        .parse::<i128>()
                        .unwrap(),
                )
        });
        nums.into_iter().rev().map(|n| n.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest_number(vec![432, 43243]),
            "43243432".to_string()
        );
    }
}
