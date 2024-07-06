// 2582. Pass the Pillow

pub struct Solution;

impl Solution {
    pub fn pass_the_pillow(mut n: i32, time: i32) -> i32 {
        n -= 1;
        if (time / n) % 2 == 1 {
            n - (time % n) + 1
        } else {
            (time % n) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
    }
}
