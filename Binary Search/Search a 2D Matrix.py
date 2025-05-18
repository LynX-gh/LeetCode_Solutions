class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m = len(matrix)-1
        n = len(matrix[0])-1

        lo = 0
        hi = m
        res_i = 0
        while lo <= hi:
            j = lo + (hi - lo)//2

            if matrix[j][-1] < target:
                lo = j+1
            elif matrix[j][-1] >= target:
                res_i = j
                hi = j-1
    
        lo = 0
        hi = n
        while lo <= hi:
            j = lo + (hi - lo)//2

            if matrix[res_i][j] < target:
                lo = j + 1
            elif matrix[res_i][j] > target:
                hi = j - 1
            else:
                return True
        return False
