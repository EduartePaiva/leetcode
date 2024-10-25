// 1233. Remove Sub-Folders from the Filesystem
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut not_sub_folders: HashSet<String> = HashSet::new();

        for path in folder {
            let mut partial_path = String::with_capacity(path.len());
            let mut contains = false;
            for sub_path in path.split('/') {
                partial_path.push_str(sub_path);
                if not_sub_folders.contains(&partial_path) {
                    contains = true;
                    break;
                }
                partial_path.push('/');
            }
            if !contains {
                not_sub_folders.insert(path);
            }
        }
        not_sub_folders.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut lst1 = Solution::remove_subfolders(vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ]);
        let mut lst2 = vec!["/a", "/c/f", "/c/d"];
        lst1.sort_unstable();
        lst2.sort_unstable();
        assert_eq!(lst1, lst2)
    }
}
