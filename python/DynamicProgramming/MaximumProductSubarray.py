class Solution:
    def maxProduct(self, nums: list[int]) -> int:
        res = max(nums)
        curMin,curMax = 1

        for n in nums:
            tmp = curMax * n
            curMax = max(n * curMin, n*curMax, n)
            curMin = min(n * curMin, tmp, n)
            res = max(curMax,res)
        return res

s = Solution()
print(s.maxProduct([1,2,3,4,10]))
