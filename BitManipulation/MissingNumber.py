class Solution:
    def missingNumber(self, nums: list[int]) -> int:
        res = len(nums)
        for i,num in enumerate(nums):
            res^=i
            res^=num
        return res
        

s = Solution()
print(s.missingNumber([3,0,1]))