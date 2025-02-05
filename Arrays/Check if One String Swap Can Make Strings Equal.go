func areAlmostEqual(s1 string, s2 string) bool {
    sw := [2]byte{0, 0}

    for i := 0; i < len(s1); i++ {
        if s1[i] != s2[i] {
            if sw[0] == 0 {
                sw[0] = s2[i]
                sw[1] = s1[i]
            } else if sw[0] != 1 && sw[0] == s1[i] && sw[1] == s2[i] {
                sw[0] = 1
            } else {
                return false
            }
        }
    }
    return sw[0] == 0 || sw[0] == 1
}