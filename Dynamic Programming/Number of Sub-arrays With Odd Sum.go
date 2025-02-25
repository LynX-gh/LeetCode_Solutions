func numOfSubarrays(arr []int) int {
    odd_sum := 0
    even_sum := 0
    res := 0
    prefix := 0

    for _, val := range arr {
        prefix += val
        if (prefix & 1 == 1) {
            odd_sum += 1
            res += even_sum + 1
        } else {
            even_sum += 1
            res += odd_sum
        }
    }

    return res % (1e9 + 7)
}