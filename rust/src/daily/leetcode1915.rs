// 1915. Number of Wonderful Substrings
pub struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();
        let mut map = [0; 1 << 10];
        map[0] = 1;
        let mut res = 0;
        let mut mask: usize = 0;
        for i in 0..word.len() {
            mask ^= 1 << (word[i] - b'a');
            res += map[mask];
            map[mask] += 1;
            for i in 0..10 {
                res += map[mask ^ (1 << i)];
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
        assert_eq!(Solution::wonderful_substrings(String::from("aba")), 4);
    }
}
