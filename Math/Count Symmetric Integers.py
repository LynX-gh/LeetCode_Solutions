class Solution:
    def countSymmetricIntegers(self, low: int, high: int) -> int:
        res = 0
        
        cur = low
        while cur <= high:
            str_cur = str(cur)
            if len(str_cur) % 2 == 1:
                cur = 10**len(str_cur)+1
                continue

            s1, s2 = str_cur[:len(str_cur)//2], str_cur[len(str_cur)//2:]

            if sum([int(x) for x in s1]) == sum([int(x) for x in s2]):
                res += 1
            cur += 1
        return res
