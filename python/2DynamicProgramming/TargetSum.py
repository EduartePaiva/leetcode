class Solution:
    def findTargetSumWays(self, nums: list[int], target: int) -> int:
        cache = {}
        def dfs(i,sum):
            if i == len(nums):
                return 1 if sum ==target else 0
            if (i,sum) in cache:
                return cache[(i,sum)]
            
            # I can add or subtract current sum
            res = dfs(i+1,sum+nums[i])
            res += dfs(i+1,sum-nums[i])
            cache[(i,sum)] = res

            return res
        return dfs(0,0)



s = Solution()
print(s.findTargetSumWays([1,1,1,1,1],3))
        