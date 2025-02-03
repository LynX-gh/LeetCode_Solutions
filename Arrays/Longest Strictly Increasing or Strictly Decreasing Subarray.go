func longestMonotonicSubarray(nums []int) int {
    res := 0;
    inc := 1;
    dec := 1;

    for i := 1; i < len(nums); i++ {
        if nums[i] < nums[i-1] {
            dec += 1
            res = max(res, inc)
            inc = 1
        } else if nums[i] > nums[i-1] {
            inc += 1
            res = max(res, dec)
            dec = 1
        } else {
            res = max(res, dec)
            dec = 1
            res = max(res, inc)
            inc = 1
        }
    }
    return max(inc, dec, res)
}