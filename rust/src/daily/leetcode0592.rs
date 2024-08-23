// 592. Fraction Addition and Subtraction
pub struct Solution;

#[derive(Debug)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expression = expression.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut fractions: Vec<Fraction> = vec![];

        while i < expression.len() {
            // get numerator
            let is_negative = if expression[i] == '-' { true } else { false };
            if expression[i] == '+' || expression[i] == '-' {
                i += 1;
            }
            let mut numerator = 0;
            while i < expression.len() && expression[i] != '/' {
                numerator *= 10;
                numerator += expression[i].to_digit(10).unwrap() as i64;
                i += 1;
            }
            numerator *= if is_negative { -1 } else { 1 };
            i += 1;

            let mut denominator = 0;
            while i < expression.len() && expression[i] != '+' && expression[i] != '-' {
                denominator *= 10;
                denominator += expression[i].to_digit(10).unwrap() as i64;
                i += 1;
            }
            fractions.push(Fraction {
                numerator,
                denominator,
            });
        }

        fn irreducible_fraction(f: Fraction) -> Fraction {
            if f.numerator == 0 {
                return Fraction {
                    numerator: 0,
                    denominator: 1,
                };
            }
            for n in 2..=f.numerator.abs().min(f.denominator) {
                if f.numerator % n == 0 && f.denominator % n == 0 {
                    return irreducible_fraction(Fraction {
                        numerator: f.numerator / n,
                        denominator: f.denominator / n,
                    });
                }
            }
            f
        }
        let mut frac1 = fractions.pop().unwrap();
        for frac2 in fractions {
            let denominator = frac1.denominator * frac2.denominator;
            let numerator =
                (frac1.numerator * frac2.denominator) + (frac2.numerator * frac1.denominator);
            frac1 = irreducible_fraction(Fraction {
                numerator,
                denominator,
            });
        }

        String::from_iter([
            frac1.numerator.to_string(),
            "/".to_string(),
            frac1.denominator.to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".to_string()),
            "0/1".to_string()
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
            "1/3".to_string()
        )
    }
}
