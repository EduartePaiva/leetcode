// 344. Reverse String
pub struct Solution;

#[cfg(test)]

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut right = s.len() - 1;
        let mut left = 0;

        while left < right {
            let bkp = s[right];
            s[right] = s[left];
            s[left] = bkp;
            left += 1;
            right -= 1;
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut test = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut test);
        assert_eq!(test, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
