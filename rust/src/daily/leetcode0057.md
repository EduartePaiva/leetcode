# Intuition
I think the main trick to solve this problem was first inserting the new_interval to the result right from the start.

# Approach
After doing that, while iterating over intervals we have three cases to check. 

First is if the last interval in res is before the current interval. In this case we just push current interval.

Second case is if the last interval in res is after the current interval. This happens because we pushed new_interval right from the start and that's fine, in this case we insert the current interval before the last interval. (I saw some solutions trying to get around with using insert, creating flag to track if new_interval was inserted, but this just complicate things without any relevant increase in speed).

Third case that was left is if they overlap. In that case we merge them taking the min() for the start and the max() for the end. 




# Complexity
- Time complexity:
$$O(n)$$

- Space complexity:
$$O(n)$$ I could say $$O(1)$$ because the extra memory is the return value.

# Code
```
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        res.push(new_interval);

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];
            let lst_index = res.len() - 1;

            //case where don't overlap
            if res[lst_index][1] < start {
                res.push(interval);
                continue;
            }
            //case where the last is beyond the interval
            if end < res[lst_index][0] {
                res.insert(lst_index, interval);
                continue;
            }

            //case where overlap
            res[lst_index][0] = std::cmp::min(res[lst_index][0], start);
            res[lst_index][1] = std::cmp::max(res[lst_index][1], end);
        }

        res
    }
}
```