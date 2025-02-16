class Solution:
    def countAndSay(self, n: int) -> str:
        def rle(s: str) -> str:
            res = ""
            curr_ch = s[0]
            curr_cnt = 0
            for char in s:
                if char == curr_ch:
                    curr_cnt+=1
                else:
                    res += f"{curr_cnt}{curr_ch}"
                    curr_ch = char
                    curr_cnt = 1
            res += f"{curr_cnt}{curr_ch}"
            return res

        curr = ""

        for i in range(1, n+1):
            if i == 1:
                curr = "1"
            else:
                curr = rle(curr)
        return curr