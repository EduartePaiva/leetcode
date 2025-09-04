// 3516. Find Closest Person
pub struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let x_dist = (x - z).abs();
        let y_dist = (y - z).abs();

        if x_dist < y_dist {
            return 1;
        } else if y_dist < x_dist {
            return 2;
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_closest(2, 7, 4), 1)
    }
}
