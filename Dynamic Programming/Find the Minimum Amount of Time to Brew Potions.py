class Solution:
    def minTime(self, skill: List[int], mana: List[int]) -> int:
        n = len(skill)
        m = len(mana)
        
        prefix = [0] * n
        prefix[0] = skill[0]
        for i in range(1, n):
            prefix[i] = prefix[i-1] + skill[i]
 
        X = [0] * m
        X[0] = 0

        for j in range(1, m):
            lower_bound = 0
            for i in range(n):
                if i == 0:
                    bound = X[j-1] + skill[0] * mana[j-1]
                else:
                    bound = X[j-1] + prefix[i] * mana[j-1] - (prefix[i] - skill[i]) * mana[j]
                lower_bound = max(lower_bound, bound)
            X[j] = lower_bound

        answer = X[m-1] + prefix[-1] * mana[m-1]
        return answer