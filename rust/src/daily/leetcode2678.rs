// 2678. Number of Senior Citizens
pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.into_iter().fold(0, |prev, detail| {
            let age: u8 = detail[11..13].parse().unwrap();
            if age > 60 {
                prev + 1
            } else {
                prev
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_seniors(vec![
                "7868190130M7522".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ]),
            2
        );
    }
}
