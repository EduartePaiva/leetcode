// 2191. Sort the Jumbled Numbers
pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        //consider num positive
        let mut mapped_nums = vec![];

        for &num in &nums {
            let mut cur = num as usize;
            if cur == 0 {
                mapped_nums.push(mapping[0]);
                continue;
            }
            let mut cnt = 1;
            let mut res = 0;
            while cur > 0 {
                res += cnt * mapping[cur % 10];
                cnt *= 10;
                cur /= 10;
            }
            mapped_nums.push(res);
        }
        // println!("{:?}", mapped_nums);
        let mut new_nums: Vec<_> = mapped_nums
            .into_iter()
            .enumerate()
            .zip(nums.into_iter())
            .collect();
        new_nums.sort_by(|a, b| a.0 .1.cmp(&b.0 .1).then(a.0 .0.cmp(&b.0 .0)));

        new_nums.into_iter().map(|x| x.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::sort_jumbled(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            ),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::sort_jumbled(
                vec![5, 6, 8, 7, 4, 0, 3, 1, 9, 2],
                //   0, 1, 2, 3, 4, 5, 6, 7, 8, 9
                vec![7686, 97012948, 84234023, 2212638, 99] //  [3931, 94286512, 78547849, 9738688, 22]
            ),
            vec![99, 7686, 2212638, 97012948, 84234023]
        )
    }
}
