impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.into_iter().map(
            |customer| {
                customer.into_iter().sum()
            }
        ).max().unwrap()
    }
}

/* impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut res:i32 = 0;
        for customer in accounts {
            let wealth:i32 = customer.iter().sum();
            if wealth > res {
                res = wealth;
            }
        }
        return res;
    }
} */