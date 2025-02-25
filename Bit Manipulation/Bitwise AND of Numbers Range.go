func rangeBitwiseAnd(left int, right int) int {
    for right-1 >= left {
        right &= right - 1
    }
    return right
}