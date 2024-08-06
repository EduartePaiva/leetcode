// 3016. Minimum Number of Pushes to Type Word II
pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let word = word.as_bytes();
        let mut letter_cnt = vec![0; 26];
        for &c in word {
            letter_cnt[(c - b'a') as usize] += 1;
        }
        let mut letter_cnt = letter_cnt
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        letter_cnt.sort_by_key(|x| x.1);
        letter_cnt.reverse();

        let mut letter_cost = vec![0; 26];
        for i in 0..letter_cnt.len() {
            letter_cost[letter_cnt[i].0] = i as i32 / 8 + 1;
        }
        let mut res = 0;
        for (c, qtd) in letter_cnt {
            res += qtd * letter_cost[c];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_pushes(String::from("abcde")), 5);
    }
}
