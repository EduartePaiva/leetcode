// 791. Custom Sort String
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut word_order = HashMap::with_capacity(order.len());
        for (index, char) in order.chars().enumerate() {
            word_order.insert(char, index);
        }
        let mut vec_to_sort: Vec<char> = s.chars().collect();
        vec_to_sort.sort_by_key(|val| word_order.get(val).unwrap_or(&usize::MAX));

        return vec_to_sort.iter().collect();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::custom_sort_string(String::from("cba"), String::from("abcd")),
            String::from("cbad")
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::custom_sort_string(String::from("hucw"), String::from("utzoampdgkalexslxoqfkdjoczajxtuhqyxvlfatmptqdsochtdzgypsfkgqwbgqbcamdqnqztaqhqanirikahtmalzqjjxtqfnh")),
            String::from("hhhhhuucccwtzoampdgkalexslxoqfkdjozajxtqyxvlfatmptqdsotdzgypsfkgqbgqbamdqnqztaqqanirikatmalzqjjxtqfn")
        )
    }
}
