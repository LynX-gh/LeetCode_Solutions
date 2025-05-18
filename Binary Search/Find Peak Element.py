class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        hi = len(nums)-1
        lo = 0

        while lo < hi:
            mid = lo + (hi - lo)//2

            if nums[mid] < nums[mid+1]:
                lo = mid + 1
            else:
                hi = mid
        return lo