// 2516. Take K of Each Character From Left and Right
pub struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let mut cnt = [0, 0, 0];
        let s = s.as_bytes();
        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
        }
        if cnt[0] < k || cnt[1] < k || cnt[2] < k {
            return -1;
        } else if cnt[0] == k && cnt[1] == k && cnt[2] == k {
            return s.len() as i32;
        }
        let mut res = s.len();
        let mut l = 0;
        for r in 0..s.len() {
            cnt[(s[r] - b'a') as usize] -= 1;
            while cnt[0] < k || cnt[1] < k || cnt[2] < k {
                cnt[(s[l] - b'a') as usize] += 1;
                l += 1;
            }
            res = res.min(s.len() - (r + 1 - l));
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::take_characters("abc".to_string(), 1), 3);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::take_characters("cbbac".to_string(), 1), 3);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::take_characters("bcca".to_string(), 1), 3);
    }
}
