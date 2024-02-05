class Solution:
    def lengthOfLIS(self, nums: list[int]) -> int:
        mem = [1] * len(nums)
        res = 1

        for i in range(len(nums)-1,-1,-1):
            for j in range(i+1,len(nums)):
                if nums[i] < nums[j]:
                    mem[i] = max(mem[i],mem[j]+1)
            res = max(res,mem[i])
        return res




s = Solution()
print(s.lengthOfLIS([0,1,0,3,2,3]))
        