impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut start = right;

        while start-1 >= left {
            start &= start - 1;
        }
        start
    }
}