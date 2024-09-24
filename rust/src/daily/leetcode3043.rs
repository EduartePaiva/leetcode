// 3043. Find the Length of the Longest Common Prefix
pub struct Solution;

#[derive(Clone)]
struct Trie {
    children: Vec<Option<Trie>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: vec![None; 10],
        }
    }

    fn convert_num_to_vec(mut num: i32) -> Vec<usize> {
        let mut res = vec![];
        while num > 0 {
            res.push((num % 10) as usize);
            num /= 10;
        }
        res
    }

    pub fn insert(&mut self, num: i32) {
        let mut head = &mut self.children;
        let mut items = Trie::convert_num_to_vec(num);
        while let Some(num) = items.pop() {
            if head[num].is_none() {
                head[num] = Some(Trie::new());
            }
            head = &mut head[num].as_mut().unwrap().children;
        }
    }

    pub fn prefix_size(&self, num: i32) -> i32 {
        let mut vec_num = Trie::convert_num_to_vec(num);
        let mut size = 0;
        let mut head = &self.children;
        while let Some(n) = vec_num.pop() {
            if let Some(child) = &head[n] {
                head = &child.children;
                size += 1;
            } else {
                break;
            }
        }
        size
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let mut res = 0;
        for n in arr1 {
            trie.insert(n);
        }
        for n in arr2 {
            res = res.max(trie.prefix_size(n));
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
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
    }
}
