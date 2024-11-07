// 2275. Largest Combination With Bitwise AND Greater Than Zero
pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut cnt_bits = [0; 27];
        for n in candidates {
            for b in 0..27 {
                if n & (1 << b) != 0 {
                    cnt_bits[b] += 1;
                }
            }
        }
        cnt_bits.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
            4
        );
    }
}
