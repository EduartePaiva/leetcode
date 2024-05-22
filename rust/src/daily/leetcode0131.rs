// 131. Palindrome Partitioning
pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s: Vec<char> = s.chars().collect();
        let mut res: Vec<Vec<String>> = vec![];

        fn is_palindrome(s: &[char]) -> bool {
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                if s[right] != s[left] {
                    return false;
                }
                right -= 1;
                left += 1;
            }
            return true;
        }

        fn dfs(i: usize, s: &Vec<char>, acc: Vec<String>, res: &mut Vec<Vec<String>>) {
            if i == s.len() {
                res.push(acc);
                return;
            }
            for j in (i + 1)..=s.len() {
                if is_palindrome(&s[i..j]) {
                    let mut new_acc = acc.clone();
                    new_acc.push(s[i..j].iter().collect());
                    dfs(j, s, new_acc, res);
                }
            }
        }

        dfs(0, &s, vec![], &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition(String::from("aab")),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::partition(String::from("efe")),
            vec![vec!["e", "f", "e"], vec!["efe"]]
        )
    }
}
