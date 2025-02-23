class Solution:
    def hasSameDigits(self, s: str) -> bool:
        while len(s) > 2:
            new_s = ""
            for i in range(0, len(s)-1):
                new_s += str((int(s[i]) + int(s[i+1])) % 10)
            s = new_s
        return new_s[0] == new_s[1]