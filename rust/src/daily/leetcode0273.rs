// 273. Integer to English Words
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let words = HashMap::from([
            (0, ""),
            (1, "One"),
            (2, "Two"),
            (3, "Three"),
            (4, "Four"),
            (5, "Five"),
            (6, "Six"),
            (7, "Seven"),
            (8, "Eight"),
            (9, "Nine"),
            (10, "Ten"),
            (11, "Eleven"),
            (12, "Twelve"),
            (13, "Thirteen"),
            (14, "Fourteen"),
            (15, "Fifteen"),
            (16, "Sixteen"),
            (17, "Seventeen"),
            (18, "Eighteen"),
            (19, "Nineteen"),
            (20, "Twenty"),
            (30, "Thirty"),
            (40, "Forty"),
            (50, "Fifty"),
            (60, "Sixty"),
            (70, "Seventy"),
            (80, "Eighty"),
            (90, "Ninety"),
        ]);

        fn get_hundred(number: i32, words: &HashMap<i32, &str>) -> String {
            if let Some(&word) = words.get(&number) {
                return String::from(word);
            }
            match number {
                21..=99 => String::from_iter([
                    get_hundred((number / 10) * 10, words),
                    " ".to_string(),
                    get_hundred(number % 10, words),
                ]),
                100..=999 => {
                    let mut res = String::from_iter([
                        get_hundred(number / 100, words),
                        " Hundred".to_string(),
                    ]);
                    if number % 100 > 0 {
                        res.extend([" ", get_hundred(number % 100, words).as_str()]);
                    }
                    res
                }
                _ => panic!("this function can't be used in numbers greater than 3 digits"),
            }
        }
        let mut res = String::from("");
        //convert billion
        let left = num / 1000000000;
        num %= 1000000000;
        let str_result = get_hundred(left, &words);
        if str_result.len() > 0 {
            res.push_str(str_result.as_str());
            res.push_str(" Billion");
        }
        //convert million
        let left = num / 1000000;
        num %= 1000000;
        let str_result = get_hundred(left, &words);
        if str_result.len() > 0 {
            if res.len() > 0 {
                res.push_str(" ");
            }
            res.push_str(str_result.as_str());
            res.push_str(" Million");
        }
        //convert thousands
        let left = num / 1000;
        num %= 1000;
        let str_result = get_hundred(left, &words);
        if str_result.len() > 0 {
            if res.len() > 0 {
                res.push_str(" ");
            }
            res.push_str(str_result.as_str());
            res.push_str(" Thousand");
        }
        //convert hundreds
        let str_result = get_hundred(num, &words);
        if str_result.len() > 0 {
            if res.len() > 0 {
                res.push_str(" ");
            }
            res.push_str(str_result.as_str());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_to_words(123),
            String::from("One Hundred Twenty Three")
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_to_words(12345),
            String::from("Twelve Thousand Three Hundred Forty Five")
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::number_to_words(1234567891),
            String::from("One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One")
        );
    }
}
