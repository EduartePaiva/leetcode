// 1255. Maximum Score Words Formed by Letters
pub struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut letters_cnt = [0; 26];
        for c in letters {
            letters_cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut word_cnt: Vec<Vec<(usize, i32)>> = vec![];
        for w in words {
            let mut map = [0; 26];
            for c in w.as_bytes() {
                map[(*c - b'a') as usize] += 1;
            }
            word_cnt.push(map.into_iter().enumerate().filter(|x| x.1 != 0).collect())
        }

        // now I can either chose to add current word or skip it
        // before adding current word I have to check if it's possible to add it.
        fn dfs(
            word_cnt: &Vec<Vec<(usize, i32)>>,
            letters_cnt: &mut [i32; 26],
            score: &Vec<i32>,
            word_i: usize,
        ) -> i32 {
            if word_cnt.len() == word_i {
                return 0;
            }
            let mut res = 0;
            // add current word
            //check if I can add current word
            let cur_word = &word_cnt[word_i];
            let mut can_add = true;
            for &(i, cnt) in cur_word {
                if letters_cnt[i] < cnt {
                    can_add = false;
                    break;
                }
            }
            if can_add {
                for &(i, cnt) in cur_word {
                    res += cnt * score[i];
                    letters_cnt[i] -= cnt;
                }
                res += dfs(word_cnt, letters_cnt, score, word_i + 1);
                for &(i, cnt) in cur_word {
                    letters_cnt[i] += cnt;
                }
            }
            // skip cur word
            res = res.max(dfs(word_cnt, letters_cnt, score, word_i + 1));

            res
        }
        dfs(&word_cnt, &mut letters_cnt, &score, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "dog".to_string(),
                    "cat".to_string(),
                    "dad".to_string(),
                    "good".to_string()
                ],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
    }
}
