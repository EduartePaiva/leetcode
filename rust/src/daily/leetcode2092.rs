// 2092. Find All People With Secret
pub struct Solution;

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut meetings: Vec<(i32, usize, usize)> = meetings
            .iter()
            .map(|val| (val[2], val[0] as usize, val[1] as usize))
            .collect();
        // (time,personX,personY)
        meetings.sort_unstable();
        let mut nodes: Vec<usize> = (0..n as usize).collect();
        nodes[first_person as usize] = 0;
        let mut times_meeting: Vec<&[(i32, usize, usize)]> = Vec::new();

        let mut i = 0;
        while i < meetings.len() {
            let cur_time = meetings[i].0;
            let mut end = i + 1;
            while end < meetings.len() && meetings[end].0 == cur_time {
                end += 1;
            }
            times_meeting.push(&meetings[i..end]);
            i = end;
        }

        fn find(mut x: usize, nodes: &mut Vec<usize>) -> usize {
            while x != nodes[x] {
                nodes[x] = nodes[nodes[x]];
                x = nodes[x];
            }
            x
        }
        fn union(x: usize, y: usize, nodes: &mut Vec<usize>) {
            let parent_x = find(x, nodes);
            let parent_y = find(y, nodes);

            if parent_x == 0 {
                nodes[parent_y] = 0;
            } else {
                nodes[parent_x] = parent_y;
            }
        }

        for same_time_meets in times_meeting {
            for (_, x, y) in same_time_meets {
                union(*x, *y, &mut nodes);
            }
            for (_, x, y) in same_time_meets {
                if find(*x, &mut nodes) != 0 {
                    nodes[*x] = *x;
                    nodes[*y] = *y;
                }
            }
        }

        (0..nodes.len() as i32)
            .filter(|x| find(*x as usize, &mut nodes) == 0)
            .collect()
    }
}
