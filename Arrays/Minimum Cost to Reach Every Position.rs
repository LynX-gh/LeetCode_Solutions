impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; cost.len()];
        let mut min_front = i32::MAX;

        for i in 0..cost.len() {
            if cost[i] < min_front {
                min_front = cost[i];
            }
            res[i] = min_front;
        }
        res
    }
}
