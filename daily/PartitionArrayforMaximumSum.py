class Solution:
    def maxSumAfterPartitioning(self, arr: list[int], k: int) -> int:
        dp = {}
        def dfs(i):
            if i in dp:
                return dp[i]
            
            cur_max = 0
            res = 0
            for j in range(i, min(len(arr), i+k)):
                cur_max = max(arr[j],cur_max)
                cur_val = (j+1-i) * cur_max
                res = max(res,dfs(j+1)+cur_val)

            dp[i] = res
            return res
        
        return dfs(0)



        


s = Solution()
print(s.maxSumAfterPartitioning([1,15,7,9,2,5,10],3))