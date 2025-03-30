class Solution:
    def longestPalindrome(self, s: str, t: str) -> int:
        def is_palindrome(s):
            left, right = 0, len(s) - 1
            while left < right:
                if s[left] != s[right]:
                    return False
                left += 1
                right -= 1
            return True
        
        max_length = 0
    
        for i in range(len(s)):
            for j in range(i + 1, len(s) + 1):
                substring_s = s[i:j]
                
                for k in range(len(t)):
                    for l in range(k + 1, len(t) + 1):
                        substring_t = t[k:l]
    
                        concatenated = substring_s + substring_t
                        if is_palindrome(concatenated):
                            max_length = max(max_length, len(concatenated))
    
        for i in range(len(s)):
            for j in range(i + 1, len(s) + 1):
                substring_s = s[i:j]
                if is_palindrome(substring_s):
                    max_length = max(max_length, len(substring_s))
    
        for k in range(len(t)):
            for l in range(k + 1, len(t) + 1):
                substring_t = t[k:l]
                if is_palindrome(substring_t):
                    max_length = max(max_length, len(substring_t))
        
        return max_length
