//787. Cheapest Flights Within K Stops
pub struct Solution;

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn find_cheapest_price(_n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut prices: HashMap<i32, i32> = HashMap::new();
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
        deque.push_back((0, src));
        prices.insert(src, 0);

        for flight in flights {
            adj.entry(flight[0])
                .or_insert(Vec::new())
                .push((flight[2], flight[1]));
        }

        for _ in 0..k + 1 {
            for _ in 0..deque.len() {
                let (cost, node) = deque.pop_front().unwrap();
                if let Some(values) = adj.get(&node) {
                    for (price, to) in values {
                        let new_price = cost + price;
                        if !prices.contains_key(to) {
                            prices.insert(*to, new_price);
                            deque.push_back((new_price, *to));
                        } else if *prices.get(to).unwrap() > new_price {
                            prices.insert(*to, new_price);
                            deque.push_back((new_price, *to));
                        }
                    }
                }
            }
            if deque.len() == 0 {
                break;
            }
        }

        *prices.entry(dst).or_insert(-1)
    }
}
