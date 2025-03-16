class Solution:
    def solveQueries(self, nums: List[int], queries: List[int]) -> List[int]:
        n = len(nums)

        idx_map = {}
        for i, val in enumerate(nums):
            if val not in idx_map:
                idx_map[val] = []
            idx_map[val].append(i)
        
        res = []
        for q in queries:
            v = nums[q]
            positions = idx_map[v]

            if len(positions) == 1:
                res.append(-1)
                continue

            pos = bisect_left(positions, q)
            prev_index = positions[pos - 1] if pos > 0 else positions[-1]
            next_index = positions[pos + 1] if pos < len(positions) - 1 else positions[0]

            d_prev = min(abs(q - prev_index), n - abs(q - prev_index))
            d_next = min(abs(q - next_index), n - abs(q - next_index))
            
            res.append(min(d_prev, d_next))
            
        return res