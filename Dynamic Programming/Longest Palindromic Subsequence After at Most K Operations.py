class Solution:
    def longestPalindromicSubsequence(self, s: str, k: int) -> int:
        n = len(s)
        memo = {}

        def min_shift_cost(a, b):
            return min((ord(b) - ord(a)) % 26, (ord(a) - ord(b)) % 26)

        def lps_with_ops(i: int, j: int, ops_left: int) -> int:
            if i > j:
                return 0
            if i == j:
                return 1
            
            if (i, j, ops_left) in memo:
                return memo[(i, j, ops_left)]

            if s[i] == s[j]:
                result = 2 + lps_with_ops(i + 1, j - 1, ops_left)
            else:
                result = max(lps_with_ops(i + 1, j, ops_left), lps_with_ops(i, j - 1, ops_left))

                shift_cost = min_shift_cost(s[i], s[j])
                if ops_left >= shift_cost:
                    result = max(result, 2 + lps_with_ops(i + 1, j - 1, ops_left - shift_cost))

            memo[(i, j, ops_left)] = result
            return result
        
        return lps_with_ops(0, n - 1, k)