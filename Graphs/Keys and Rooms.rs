use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(0);

        while let Some(room) = q.pop_front() {
            visited.insert(room);
            for &key in rooms[room].iter() {
                if visited.insert(key as usize) {
                    q.push_back(key as usize);
                }
            }
        }
        visited.len() == rooms.len()
    }
}