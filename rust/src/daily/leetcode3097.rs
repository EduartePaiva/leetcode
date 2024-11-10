// 3097. Shortest Subarray With OR at Least K II
pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        // sliding window
        let mut shortest_dist = i32::MAX;
        let mut bit_map = [0; 32];

        let mut left = 0;

        fn insert_n(n: i32, bit_map: &mut [i32; 32]) -> i32 {
            let mut num = 0;

            for bit in 0..32 {
                if n & (1 << bit) > 0 {
                    bit_map[bit] += 1;
                }
                if bit_map[bit] > 0 {
                    num |= 1 << bit;
                }
            }
            num
        }
        fn remove_n(n: i32, bit_map: &mut [i32; 32]) -> i32 {
            let mut num = 0;

            for bit in 0..32 {
                if n & (1 << bit) > 0 {
                    bit_map[bit] -= 1;
                }
                if bit_map[bit] > 0 {
                    num |= 1 << bit;
                }
            }
            num
        }

        for r in 0..nums.len() {
            if insert_n(nums[r], &mut bit_map) >= k {
                shortest_dist = shortest_dist.min(((r - left) + 1) as i32);
                while left < r {
                    let remove = remove_n(nums[left], &mut bit_map);
                    left += 1;
                    if remove < k {
                        break;
                    }
                    shortest_dist = shortest_dist.min(((r - left) + 1) as i32);
                }
            }
        }

        if shortest_dist == i32::MAX {
            -1
        } else {
            shortest_dist
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }
    #[test]

    fn test2() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }
}
