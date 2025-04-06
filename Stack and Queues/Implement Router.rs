use std::collections::{HashSet, VecDeque};

struct Router {
    queue: VecDeque<(i32, i32, i32)>,
    seen: HashSet<(i32, i32, i32)>,
    max: usize,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            queue: VecDeque::new(),
            seen: HashSet::new(),
            max: memory_limit as usize,
        }
    }
    
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = (timestamp, source, destination);
        
        if self.seen.contains(&packet) {
            return false;
        }

        if self.queue.len() >= self.max {
            if let Some(oldest) = self.queue.pop_front() {
                self.seen.remove(&oldest);
            }
        }

        self.seen.insert(packet);
        self.queue.push_back(packet);
        
        true
    }
    
    fn forward_packet(&mut self) -> Vec<i32> {
        let mut res = Vec::new();
        
        if let Some((ts, src, dst)) = self.queue.pop_front() {
            res.push(src);
            res.push(dst);
            res.push(ts);

            self.seen.remove(&(ts, src, dst));
        }
        
        res
    }
    
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        self.queue.iter()
            .filter(|&&(ts, _, dst)| ts >= start_time && ts <= end_time && dst == destination)
            .count() as i32
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
