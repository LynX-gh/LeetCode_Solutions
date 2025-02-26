func maxAbsoluteSum(nums []int) int {
    curr_max := 0
    curr_min := 0
    kadane_max := 0
    kadane_min := 0

    for _, val := range nums {
        curr_max = max(0, curr_max+val)
        curr_min = min(0, curr_min+val)

        kadane_max = max(kadane_max, curr_max)
        kadane_min = min(kadane_min, curr_min)
    }

    return max(kadane_max, -kadane_min)
}