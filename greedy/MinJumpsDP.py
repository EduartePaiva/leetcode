class Solution:
    def jump(self, nums: list[int]) -> int:
        dp = [len(nums)] * len(nums)
        dp[-1] = 0

        for i in range(len(nums)-2,-1,-1):
            num = nums[i]
            if i + num > len(nums)-1:
                dp[i] = 1
            else:
                start = i
                end = i+num
                for j in range(end,start-1,-1):
                    dp[i] = min(dp[i], 1+dp[j])
                    if dp[i] == 2: break
        return dp[0]




s = Solution()
print(s.jump([2,3,1,1,4]))