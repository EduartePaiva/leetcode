class Solution:
    def maxSubArray(self, nums: list[int]) -> int:
        maxSum = nums[0]
        prevSum = 0

        for num in nums:
            if prevSum < 0:prevSum = 0
            prevSum+=num
            if prevSum > maxSum: maxSum = prevSum
        return maxSum

s = Solution()
print(s.maxSubArray([-2,1,-3,4,-1,2,1,-5,4]))