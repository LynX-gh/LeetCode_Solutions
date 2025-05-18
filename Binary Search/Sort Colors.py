class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        zero_ptr = 0
        two_ptr = len(nums)-1
        mid = 0

        while mid <= two_ptr:
            if nums[mid] == 0:
                nums[mid], nums[zero_ptr] = nums[zero_ptr], nums[mid]
                zero_ptr += 1
                mid += 1
            elif nums[mid] == 2:
                nums[mid], nums[two_ptr] = nums[two_ptr], nums[mid]
                two_ptr -= 1
            else:
                mid += 1
