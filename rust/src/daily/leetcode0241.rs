// 241. Different Ways to Add Parentheses
pub struct Solution;

#[derive(Debug)]
enum Sig {
    Plus,
    Minus,
    Star,
}

#[derive(Debug)]
enum PExp {
    Num(i32),
    Exp(Sig),
}

use PExp::*;
use Sig::*;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let expression = expression.as_bytes();
        let mut n_exp: Vec<PExp> = vec![];
        let mut num = 0;
        for i in 0..expression.len() {
            match expression[i] {
                b'+' => {
                    n_exp.push(Num(num));
                    num = 0;
                    n_exp.push(Exp(Plus));
                }
                b'-' => {
                    n_exp.push(Num(num));
                    num = 0;
                    n_exp.push(Exp(Minus));
                }
                b'*' => {
                    n_exp.push(Num(num));
                    num = 0;
                    n_exp.push(Exp(Star));
                }
                _ => {
                    num *= 10;
                    num += (expression[i] - b'0') as i32;
                }
            }
        }
        n_exp.push(Num(num));

        fn backtrack(n_exp: &Vec<PExp>, start: usize, end: usize) -> Vec<i32> {
            if end - start == 1 {
                if let Num(n) = n_exp[start] {
                    return vec![n];
                }
            }
            let mut res = vec![];
            for i in start..end {
                match &n_exp[i] {
                    Num(_) => continue,
                    Exp(signal) => {
                        let right = backtrack(n_exp, start, i);
                        let left = backtrack(n_exp, i + 1, end);
                        for a in right {
                            for b in &left {
                                res.push(match signal {
                                    Minus => a - b,
                                    Plus => a + b,
                                    Star => a * b,
                                });
                            }
                        }
                    }
                }
            }
            return res;
        }

        backtrack(&n_exp, 0, n_exp.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::diff_ways_to_compute("2-1-1".to_string()),
            vec![2, 0]
        )
    }
}
