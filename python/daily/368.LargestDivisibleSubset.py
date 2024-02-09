class Solution:
    def largestDivisibleSubset(self, nums: list[int]) -> list[int]:
        nums.sort()
        dp = {} #(i,prev) -> []

        def dfs(i,prev):
            if i == len(nums):
                return []
            if (i,prev) in dp:
                return dp[(i,prev)]
            res = []
            #include
            if nums[i] % prev == 0:
                res.append(nums[i])
                res.extend(dfs(i+1,nums[i]))
            #exclude
            tmp = dfs(i+1,prev)

            res = tmp if len(tmp) > len(res) else res
            dp[(i,prev)] = res
            return res
        return dfs(0,1)

s = Solution()
print(s.largestDivisibleSubset([1,2,3]))

            

