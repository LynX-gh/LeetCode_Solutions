func maxProfit(prices []int) int {
    low := prices[0]
    res := 0

    for i := 0; i < len(prices); i++ {
        low = min(prices[i], low)
        res = max(prices[i] - low, res)
    }
    return res;
}