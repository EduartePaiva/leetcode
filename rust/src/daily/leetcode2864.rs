// 2864. Maximum Odd Binary Number
pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut res = String::with_capacity(s.len());

        let mut num_of_1 = 0;
        for c in s.chars() {
            if c == '1' {
                num_of_1 += 1;
            }
        }
        let num_of_0 = s.len() - num_of_1;

        res.extend((0..num_of_1 - 1).map(|_| '1'));
        res.extend((0..num_of_0).map(|_| '0'));
        res.push('1');

        res
    }
}

#[test]
fn maximum_odd_binary_number() {
    assert_eq!(
        Solution::maximum_odd_binary_number(String::from("010"),),
        String::from("001")
    );
    assert_eq!(
        Solution::maximum_odd_binary_number(String::from("0101"),),
        String::from("1001")
    );
}
