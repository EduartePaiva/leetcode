// 2709. Greatest Common Divisor Traversal
pub struct Solution;

use std::collections::HashMap;
struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        Self {
            count: n as i32,
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.par[x] {
            self.par[x] = self.par[self.par[x]];
            x = self.par[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.size[root_x] > self.size[root_y] {
            self.par[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.par[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        }
        self.count -= 1;
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut prime_numbers: HashMap<i32, usize> = HashMap::new();
        // (prime num, last_found)
        let mut uf = UnionFind::new(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let mut f = 2;
            let mut new_num = *num;
            while f * f <= new_num {
                if new_num % f == 0 {
                    while new_num % f == 0 {
                        new_num /= f;
                    }
                    match prime_numbers.get(&f) {
                        Some(i2) => uf.union(i, *i2),
                        None => {
                            prime_numbers.insert(f, i);
                        }
                    }
                }
                f += 1;
            }
            if new_num != 1 {
                match prime_numbers.get(&new_num) {
                    Some(i2) => uf.union(i, *i2),
                    None => {
                        prime_numbers.insert(new_num, i);
                    }
                }
            }
        }

        uf.count == 1
    }
}
