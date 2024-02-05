class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        dp = {}

        def dfs(i,buying):
            if i >= len(prices):
                return 0
            if (i,buying) in dp:
                return dp[(i,buying)]
            
            if buying:
                buy = dfs(i+1, not buying) - prices[i]
                cd = dfs(i+1, buying)
                mx = max(buy,cd)
                dp[(i,buying)] = mx
                return mx
            else:
                sell = dfs(i+2, not buying) + prices[i]
                cd = dfs(i+1,buying)
                mx = max(sell,cd)
                dp[(i,buying)] = mx
                return mx
        return dfs(0,True)
            

    
s = Solution()
print(s.maxProfit([2,1,4]))
print(s.maxProfit([1,2,3,0,2]))
        