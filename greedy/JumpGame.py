class Solution:
    def canJump(self, nums: list[int]) -> bool:
        prevJump = 1
        for i in range(len(nums)-1):
            prevJump -= 1
            if nums[i] > prevJump: prevJump = nums[i]
            if prevJump == 0: return False
        return True
        

s = Solution()
print(s.canJump([3,2,1,0,0]))