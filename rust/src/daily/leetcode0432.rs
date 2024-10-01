//All O`one Data Structure

#![allow(unused)]
use std::collections::HashMap;
struct AllOne {
    mapping: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        *self.mapping.entry(key).or_insert(0) += 1;
    }

    fn dec(&mut self, key: String) {
        let ent = self.mapping.entry(key.clone()).or_insert(0);
        *ent -= 1;
        if *ent <= 0 {
            self.mapping.remove(&key);
        }
    }

    fn get_max_key(&self) -> String {
        // find the biggest value of the hashmap
        let mut max_key = String::new();
        let mut max_value = 0;
        for (key, value) in self.mapping.iter() {
            if *value > max_value {
                max_key = key.clone();
                max_value = *value;
            }
        }
        max_key
    }

    fn get_min_key(&self) -> String {
        let mut min_key = String::new();
        let mut min_value = i32::MAX;
        for (key, value) in self.mapping.iter() {
            if *value < min_value {
                min_key = key.clone();
                min_value = *value;
            }
        }
        min_key
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        all_one.inc("hello".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "hello".to_string()); // return "hello"
        all_one.inc("leet".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string()); // return "hello"
        assert_eq!(all_one.get_min_key(), "leet".to_string()); // return "leet"
    }
}
