// 860. Lemonade Change
pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut num_05_bill = 0;
        let mut num_10_bill = 0;

        for bill in bills {
            match bill {
                5 => num_05_bill += 1,
                10 => {
                    if num_05_bill == 0 {
                        return false;
                    }
                    num_05_bill -= 1;
                    num_10_bill += 1;
                }
                20 => {
                    if num_10_bill > 0 && num_05_bill > 0 {
                        num_05_bill -= 1;
                        num_10_bill -= 1;
                    } else if num_05_bill >= 3 {
                        num_05_bill -= 3;
                    } else {
                        return false;
                    }
                }
                _ => panic!("this can't happen"),
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }
}
