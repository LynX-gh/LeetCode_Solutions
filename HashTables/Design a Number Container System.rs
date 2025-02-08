use std::collections::{HashMap, BTreeSet};

struct NumberContainers {
    index_map: HashMap<i32, i32>,
    min_map: HashMap<i32, BTreeSet<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {

    fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            min_map: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.index_map.get(&index) {
            if let Some(set) = self.min_map.get_mut(&old_number) {
                set.remove(&index);
                if set.is_empty() {
                    self.min_map.remove(&old_number);
                }
            }
        }

        self.index_map.insert(index, number);
        self.min_map
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        if let Some(set) = self.min_map.get(&number) {
            if let Some(&val) = set.iter().next() {
                return val;
            }
        }
        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */