func minimumRecolors(blocks string, k int) int {
    res := 101
    curr := 0
    start := 0

    for i, ch := range blocks {
        if ch == 'W' {
            curr += 1
        }
        if i - start + 1 > k {
            if blocks[start] == 'W' {
                curr -= 1
            }
            start += 1
        }
        if i - start + 1 == k {
            res = min(res, curr)
        }
    }
    return res
}