// 3011. Find if Array Can Be Sorted
pub struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut nums_stack: Vec<i32> = vec![];
        for n in nums {
            let mut popped_vals: Vec<i32> = vec![];
            while !nums_stack.is_empty() && *nums_stack.last().unwrap() > n {
                let num = nums_stack.pop().unwrap();
                if num.count_ones() != n.count_ones() {
                    return false;
                }
                popped_vals.push(num);
            }
            nums_stack.push(n);
            nums_stack.extend(popped_vals.into_iter().rev());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::can_sort_array(vec![18, 3, 8]), false);
    }
}
