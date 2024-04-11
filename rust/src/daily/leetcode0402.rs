// 402. Remove K Digits

pub struct Solution;

use std::char;
impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        // monotonic stack
        // clean up leading zeros
        // if there's k left I remove k from the ms
        let num: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut monotonic_stack = Vec::with_capacity(num.len());

        for n in num {
            if monotonic_stack.is_empty() {
                monotonic_stack.push(n);
                continue;
            }
            while k > 0 && !monotonic_stack.is_empty() && monotonic_stack.last().unwrap() > &n {
                monotonic_stack.pop();
                k -= 1;
            }
            monotonic_stack.push(n);
        }

        // remove leading 0
        let mut leading_0 = 0;
        while let Some(n) = monotonic_stack.get(leading_0) {
            if n == &0 {
                leading_0 += 1;
            } else {
                break;
            }
        }

        //println!("{:?}, {leading_0}", monotonic_stack);
        //count leading 0
        monotonic_stack
            .drain(leading_0..)
            .map(|n| char::from_digit(n, 10).unwrap())
            .collect()
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
}
