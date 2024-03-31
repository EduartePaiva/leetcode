// 2444. Count Subarrays With Fixed Bounds

pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // three pointers too
        // I have to keep track of cur min and cur amx
        // I have to keep track of count min_k and count max_k

        let mut res = 0;
        let mut cur_min = i32::MAX;
        let mut cur_max = 0;
        let mut cnt_min = 0;
        let mut cnt_max = 0;
        let mut near_l = 0;
        let mut far_l = 0;

        for r in 0..nums.len() {
            cur_min = cur_min.min(nums[r]);
            cur_max = cur_max.max(nums[r]);
            cnt_min += if nums[r] == min_k { 1 } else { 0 };
            cnt_max += if nums[r] == max_k { 1 } else { 0 };
            if nums[near_l] != min_k && nums[near_l] != max_k {
                near_l += 1;
            }

            if cur_min < min_k || cur_max > max_k {
                //here I have to shift near_l and far_l
                cur_min = i32::MAX;
                cur_max = 0;
                cnt_min = 0;
                cnt_max = 0;
                near_l = r + 1;
                far_l = r + 1;
            }

            //how can I shift the near_l?
            // the near_l have to be 1 step away from
            // taking cnt_min or cnt_max
            if cnt_min > 1 || cnt_max > 1 {
                loop {
                    let to_take = nums[near_l];
                    let cnt = if to_take == max_k {
                        &mut cnt_max
                    } else {
                        &mut cnt_min
                    };
                    //check if it can take
                    if *cnt > 1 {
                        // if took get 1 step away
                        *cnt -= 1;
                        near_l += 1;
                        while near_l < r && nums[near_l] != min_k && nums[near_l] != max_k {
                            near_l += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
            if cnt_min >= 1 && cnt_max >= 1 {
                res += (near_l - far_l + 1) as i64
            }
            // println!(
            //     "cnt_min: {cnt_min}, cnt_max: {cnt_max}, near_l: {near_l}, far_l: {far_l}, r: {r}, res: {res}"
            // );
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    689862, 297861, 946099, 25145, 946099, 647669, 863241, 886257, 946099, 25145,
                    567132, 484586, 478308, 427044, 545054, 25145, 25145, 25145, 25145, 25145
                ],
                25145,
                946099
            ),
            122
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    934372, 927845, 479424, 49441, 17167, 17167, 65553, 927845, 17167, 927845,
                    17167, 425106, 17167, 927845, 17167, 927845, 251338, 17167
                ],
                17167,
                927845
            ),
            118
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    35054, 398719, 945315, 945315, 820417, 945315, 35054, 945315, 171832, 945315,
                    35054, 109750, 790964, 441974, 552913
                ],
                35054,
                945315
            ),
            81
        );
    }
}
