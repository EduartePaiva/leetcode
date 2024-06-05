// 1002. Find Common Characters
pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut count = vec![i32::MAX; 26];

        for w in words {
            let mut new_count = vec![0; 26];
            for c in w.as_bytes() {
                let index = (c - b'a') as usize;
                new_count[index] += 1;
            }
            for i in 0..26 {
                count[i] = count[i].min(new_count[i]);
            }
        }
        let mut res = vec![];
        for i in 0..26 {
            let letter = (b'a' + i as u8) as char;
            for _ in 0..count[i] {
                res.push(String::from(letter));
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec![String::from("e"), String::from("l"), String::from("l")]
        );
    }
    #[test]
    fn test2() {
        let to_compare: Vec<String> = vec![];
        assert_eq!(
            Solution::common_chars(vec![
                "acabcddd".to_string(),
                "bcbdbcbd".to_string(),
                "baddbadb".to_string(),
                "cbdddcac".to_string(),
                "aacbcccd".to_string(),
                "ccccddda".to_string(),
                "cababaab".to_string(),
                "addcaccd".to_string()
            ]),
            to_compare
        );
    }
}
