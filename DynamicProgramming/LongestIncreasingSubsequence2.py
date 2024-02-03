class Solution:
    def lengthOfLIS(self, nums: list[int]) -> int:
        dp = {}
        res = 1

        def dfs(i:int) -> int:
            if i in dp: return dp[i]

            # I can skip or include current dp
            num = nums[i]
            while i < len(nums):
                i+=1
                if num < nums[i]:
                    dfs()
            


        
        return res




s = Solution()
print(s.lengthOfLIS([0,1,0,3,2,3]))
        