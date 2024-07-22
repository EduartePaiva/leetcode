// 2418. Sort the People
pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut names_w_h: Vec<(i32, String)> =
            heights.into_iter().zip(names.into_iter()).collect();
        names_w_h.sort_unstable();
        names_w_h.into_iter().rev().map(|x| x.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        )
    }
}
