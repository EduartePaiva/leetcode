// 1652. Defuse the Bomb
pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        // o(n) solution
        let mut res = vec![0; code.len()];
        if k == 0 {
            return res;
        }
        if k > 0 {
            // next k numbers
            let mut sum_k = 0;
            let k = k as usize;
            for i in 0..k {
                sum_k += code[i];
            }
            for i in 0..code.len() {
                // remove current i
                sum_k -= code[i];
                // add the next value
                sum_k += code[(i + k) % code.len()];
                // update current
                res[i] = sum_k;
            }
        }
        if k < 0 {
            //previous k numbers
            let mut sum_k = 0;
            let k = -k as usize;
            for i in (code.len() - k)..code.len() {
                // this is for index 0
                sum_k += code[i];
            }
            res[0] = sum_k;
            for i in 1..code.len() {
                // add previous index
                sum_k += code[i - 1];
                // remove i-k-1
                let remove_idx = if i > k {
                    i - k - 1
                } else {
                    code.len() - (k - i + 1)
                };
                sum_k -= code[remove_idx];
                res[i] = sum_k;
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
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
