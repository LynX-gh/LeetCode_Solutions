class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        res = [-1, -1]

        hi = len(nums)-1
        lo = 0

        while lo <= hi:
            mid = lo + (hi - lo)//2

            if nums[mid] == target:
                res[1] = mid
                lo = mid+1
            elif nums[mid] > target:
                hi = mid-1
            else:
                lo = mid+1
        
        lo = 0
        high = len(nums)

        while lo <= hi:
            mid = lo + (hi - lo)//2

            if nums[mid] == target:
                res[0] = mid
                hi = mid-1
            elif nums[mid] > target:
                hi = mid-1
            else:
                lo = mid+1
        
        return res