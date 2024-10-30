// 1671. Minimum Number of Removals to Make Mountain Array
pub struct Solution;

impl Solution {
    fn lis(slice: &[i32], pivot: i32) -> usize {
        let mut lis: Vec<i32> = vec![];
        for &n in slice {
            if n >= pivot {
                continue;
            }
            if let Some(&lst_v) = lis.last() {
                if n > lst_v {
                    lis.push(n);
                    continue;
                }
                if let Err(idx) = lis.binary_search(&n) {
                    lis[idx] = n;
                }
            } else {
                lis.push(n);
            }
        }
        lis.len()
    }
    fn lds(slice: &[i32], pivot: i32) -> usize {
        let mut lds: Vec<i32> = vec![];
        for &n in slice.iter().rev() {
            if n >= pivot {
                continue;
            }
            if let Some(&lst_v) = lds.last() {
                if n > lst_v {
                    lds.push(n);
                    continue;
                }
                if let Err(idx) = lds.binary_search(&n) {
                    lds[idx] = n;
                }
            } else {
                lds.push(n);
            }
        }
        lds.len()
    }

    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut max_mountain_sz = 0;

        for i in 1..nums.len() - 1 {
            let lis = Solution::lis(&nums[0..i], nums[i]);
            let lds = Solution::lds(&nums[i + 1..], nums[i]);
            if lis == 0 || lds == 0 {
                continue;
            }
            max_mountain_sz = max_mountain_sz.max(lis + lds + 1);
        }

        (nums.len() - max_mountain_sz) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }
}
