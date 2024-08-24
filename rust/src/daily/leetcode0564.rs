// 564. Find the Closest Palindrome
pub struct Solution;

impl Solution {
    pub fn nearest_palindromic(number_str: String) -> String {
        let number: i64 = number_str.parse().unwrap();
        if number <= 10 {
            return (number - 1).to_string();
        }
        if number == 11 {
            return "9".to_string();
        }

        let length = number_str.len();
        let left_half: i64 = number_str[..(length + 1) / 2].parse().unwrap();

        let palindrome_candidates = vec![
            Self::generate_palindrome_from_left(left_half - 1, length % 2 == 0),
            Self::generate_palindrome_from_left(left_half, length % 2 == 0),
            Self::generate_palindrome_from_left(left_half + 1, length % 2 == 0),
            10i64.pow((length - 1) as u32) - 1,
            10i64.pow(length as u32) + 1,
        ];

        let mut nearest_palindrome = 0;
        let mut min_difference = i64::MAX;

        for candidate in palindrome_candidates.iter() {
            if *candidate == number {
                continue;
            }
            let difference = (candidate - number).abs();
            if difference < min_difference
                || (difference == min_difference && *candidate < nearest_palindrome)
            {
                min_difference = difference;
                nearest_palindrome = *candidate;
            }
        }

        nearest_palindrome.to_string()
    }

    fn generate_palindrome_from_left(mut left_half: i64, is_even_length: bool) -> i64 {
        let mut palindrome = left_half;
        if !is_even_length {
            left_half /= 10;
        }
        while left_half > 0 {
            palindrome = palindrome * 10 + left_half % 10;
            left_half /= 10;
        }
        palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::nearest_palindromic("123".to_string()),
            "121".to_string()
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::nearest_palindromic("1".to_string()),
            "0".to_string()
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::nearest_palindromic("1234".to_string()),
            "1221".to_string()
        )
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::nearest_palindromic("10".to_string()),
            "9".to_string()
        )
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::nearest_palindromic("11".to_string()),
            "9".to_string()
        )
    }
    #[test]
    fn test6() {
        assert_eq!(
            Solution::nearest_palindromic("88".to_string()),
            "77".to_string()
        )
    }
    #[test]
    fn test7() {
        assert_eq!(
            Solution::nearest_palindromic("1283".to_string()),
            "1331".to_string()
        )
    }
    #[test]
    fn test8() {
        assert_eq!(
            Solution::nearest_palindromic("9009".to_string()),
            "8998".to_string()
        )
    }
}
