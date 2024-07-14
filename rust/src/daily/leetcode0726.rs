// 726. Number of Atoms

pub struct Solution;

#[derive(Debug)]
struct Atom {
    quantity: i32,
    symbol: String,
}

#[derive(Debug)]
enum FormulaPiece {
    Atom(Atom),
    OpenParenthesis,
    CloseParenthesis(i32),
}
use std::collections::HashMap;
use FormulaPiece::*;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut new_formula: Vec<FormulaPiece> = vec![];
        let mut prev_nums: Vec<i32> = vec![];
        let mut prev_letters: Vec<char> = vec![];

        fn insert_atom(
            new_formula: &mut Vec<FormulaPiece>,
            prev_nums: &mut Vec<i32>,
            prev_letters: &mut Vec<char>,
        ) {
            if prev_letters.len() > 0 {
                let symbol: String = prev_letters.iter().collect();
                let quantity = if prev_nums.len() == 0 {
                    1
                } else {
                    prev_nums.iter().fold(0, |prev, cur| prev * 10 + cur)
                };
                new_formula.push(Atom(Atom { quantity, symbol }));
                prev_letters.clear();
                prev_nums.clear();
            }
            if prev_nums.len() > 0 {
                if let Some(CloseParenthesis(_)) = new_formula.last() {
                    new_formula.pop();
                    new_formula.push(CloseParenthesis(
                        prev_nums.iter().fold(0, |prev, cur| prev * 10 + cur),
                    ));
                    prev_nums.clear();
                }
            }
        }

        for c in formula.chars() {
            match c {
                '(' => {
                    insert_atom(&mut new_formula, &mut prev_nums, &mut prev_letters);
                    new_formula.push(OpenParenthesis);
                }
                ')' => {
                    insert_atom(&mut new_formula, &mut prev_nums, &mut prev_letters);
                    new_formula.push(CloseParenthesis(1));
                }
                '0'..='9' => prev_nums.push(c.to_digit(10).unwrap() as i32),
                'A'..='Z' => {
                    //if it's uppercase then I know that it's a new thing.
                    insert_atom(&mut new_formula, &mut prev_nums, &mut prev_letters);
                    prev_letters.push(c);
                }
                'a'..='z' => prev_letters.push(c),
                _ => panic!("Should not happen"),
            }
        }
        insert_atom(&mut new_formula, &mut prev_nums, &mut prev_letters);
        let mut stack: Vec<FormulaPiece> = vec![];
        for form in new_formula {
            if let CloseParenthesis(num) = form {
                let mut new_stack = vec![];
                while let Some(f) = stack.pop() {
                    match f {
                        OpenParenthesis => break,
                        Atom(Atom { quantity, symbol }) => new_stack.push(Atom(Atom {
                            quantity: quantity * num,
                            symbol,
                        })),
                        _ => panic!("should not happen"),
                    }
                }
                stack.extend(new_stack.into_iter());
            } else {
                stack.push(form);
            }
        }
        let mut map: HashMap<String, i32> = HashMap::new();
        for atm in stack {
            if let Atom(Atom { quantity, symbol }) = atm {
                *map.entry(symbol).or_insert(0) += quantity;
            }
        }
        let mut tup_elements: Vec<_> = map.into_iter().collect();
        tup_elements.sort_unstable();
        tup_elements
            .into_iter()
            .map(|(mut key, qtd)| {
                if qtd > 1 {
                    key.push_str(qtd.to_string().as_str());
                }
                key
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_of_atoms("H2O".to_string()),
            "H2O".to_string()
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_of_atoms("Mg(OH)2".to_string()),
            "H2MgO2".to_string()
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
            "K4N2O14S4".to_string()
        );
    }
}
