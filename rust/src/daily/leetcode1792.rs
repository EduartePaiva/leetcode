// 1792. Maximum Average Pass Ratio
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        // the max diff will be popped first

        let mut h: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

        let prob = |a, b| (a as f64) / (b as f64);
        let dif = |a, b| prob(a + 1, b + 1) - prob(a, b);

        for class in classes {
            let pass = class[0];
            let total = class[1];

            let pass_ratio = dif(pass, total);

            h.push(((pass_ratio * 1e9) as i32, pass, total));
        }

        for _ in 0..extra_students {
            let (_, pass, total) = h.pop().unwrap();
            h.push((
                ((dif(pass + 1, total + 1) * 1e9) as i32),
                pass + 1,
                total + 1,
            ));
        }

        h.iter().map(|v| prob(v.1, v.2)).sum::<f64>() / h.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.7833333333333333
        )
    }
}
