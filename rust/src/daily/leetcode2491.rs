// 2491. Divide Players Into Teams of Equal Skill
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let team_len = skill.iter().map(|x| *x as i64).sum::<i64>() / (skill.len() / 2) as i64;

        let mut skills: HashMap<i64, i32> = HashMap::new();
        for sk in skill {
            *skills.entry(sk as i64).or_insert(0) += 1;
        }

        let mut chemistry = 0;

        // for every number in skill, there must exist another number that complement the team_len
        while !skills.is_empty() {
            let cur_skill = *skills.iter().next().unwrap().0;
            let left_skill = team_len - cur_skill;
            if left_skill < 0 || !skills.contains_key(&left_skill) {
                return -1;
            }
            let sk = skills.entry(cur_skill).or_insert(1);
            *sk -= 1;
            if *sk == 0 {
                skills.remove(&cur_skill);
            }
            let sk = skills.entry(left_skill).or_insert(1);
            *sk -= 1;
            if *sk == 0 {
                skills.remove(&left_skill);
            }

            chemistry += cur_skill * left_skill;
        }

        chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }
}
