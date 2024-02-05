class Solution:
    def maxSumAfterPartitioning(self, arr: list[int], k: int) -> int:
        dp = [0] * (len(arr)+1)

        for i in range(len(arr)-1,-1,-1):
            res = 0
            cur_max = 0
            for j in range(i,min(len(arr),i+k)):
                cur_max = max(cur_max,arr[j])
                cur_val = (j-i+1) * cur_max
                res = max(res, dp[j+1]+cur_val)
            dp[i] = res
        return dp[0]



        


s = Solution()
print(s.maxSumAfterPartitioning([1,15,7,9,2,5,10],3))