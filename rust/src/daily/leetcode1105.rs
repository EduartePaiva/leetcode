// 1105. Filling Bookcase Shelves
pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut cache = vec![0; books.len() + 1];
        for i in (0..books.len()).rev() {
            let mut cur_shelf_w = 0;
            let mut cur_shelf_h = 0;
            let mut res = i32::MAX;
            for j in i..books.len() {
                if cur_shelf_w + books[j][0] > shelf_width {
                    break;
                }
                cur_shelf_h = cur_shelf_h.max(books[j][1]);
                cur_shelf_w += books[j][0];
                res = res.min(cur_shelf_h + cache[j + 1]);
            }
            cache[i] = res;
        }
        cache[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2]
                ],
                4
            ),
            6
        );
    }
}
