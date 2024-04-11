// 402. Remove K Digits

pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        // monotonic stack
        // clean up leading zeros
        // if there's k left I remove k from the ms
        let mut monotonic_stack = vec![];
        for n in num.chars() {
            while k > 0 && !monotonic_stack.is_empty() && monotonic_stack.last().unwrap() > &n {
                monotonic_stack.pop();
                k -= 1;
            }
            monotonic_stack.push(n);
        }

        //pop from stack if k is still > 0
        while k > 0 && !monotonic_stack.is_empty() {
            monotonic_stack.pop();
            k -= 1;
        }

        // remove leading 0
        let res = monotonic_stack
            .into_iter()
            .skip_while(|c| c == &'0')
            .collect::<String>();

        // return "0" if res is empty
        if res.is_empty() {
            return "0".to_string();
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
            Solution::remove_kdigits("1432219".to_string(), 3),
            "1219".to_string()
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_kdigits("10200".to_string(), 1),
            "200".to_string()
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_kdigits("10".to_string(), 2),
            "0".to_string()
        );
    }
}
