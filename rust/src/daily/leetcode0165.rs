// 165. Compare Version Numbers
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // this part I'm splitting the string by the '.' and parsing into a vec of i32
        let mut ver1nums: Vec<i32> = version1
            .split(".")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        let mut ver2nums: Vec<i32> = version2
            .split(".")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        // this part guarantee that ver1 is always less sized than ver2 straightforward
        // and if the swap happen -1 is returned, this is important for the
        // next return logic
        let swap = if ver1nums.len() > ver2nums.len() {
            let bkp = ver1nums;
            ver1nums = ver2nums;
            ver2nums = bkp;
            -1
        } else {
            1
        };

        // check who is a bigger version in the version1 size range
        // here I'm using the swap variable to return so if swap happened
        // the answer will still be correct
        for i in 0..ver1nums.len() {
            if ver1nums[i] < ver2nums[i] {
                return -swap;
            } else if ver1nums[i] > ver2nums[i] {
                return swap;
            }
        }

        // if the remaining version2 have any value that isn't 0 then version 2 is greater
        for i in ver1nums.len()..ver2nums.len() {
            if ver2nums[i] != 0 {
                return -swap;
            }
        }

        //if the two are equal this 0 will be returned
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::compare_version(String::from("1.01"), String::from("1.001")),
            0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::compare_version(String::from("1.0"), String::from("1.0.0")),
            0
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::compare_version(String::from("0.1"), String::from("1.1")),
            -1
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::compare_version(String::from("7.5.2.4"), String::from("7.5.3")),
            -1
        );
    }
}
