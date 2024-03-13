// 2485. Find the Pivot Integer
pub struct Solution;
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let n = n as usize;
        let mut prefix_sum: Vec<i32> = vec![0; n + 1];
        for val in 1..=n {
            prefix_sum[val] = prefix_sum[val - 1] + (val as i32);
        }

        for res in 1..=n {
            let calc = prefix_sum[n] - prefix_sum[res - 1];
            if prefix_sum[res] == calc {
                return res as i32;
            } else if prefix_sum[res] > calc {
                break;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }
}
