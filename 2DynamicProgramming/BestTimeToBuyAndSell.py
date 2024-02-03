class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        LEN = len(prices)
        profit = [0] * LEN
        for i in range(LEN-1,-1,-1):
            max_profit = 0
            for j in range(i+1,LEN):
                cur_profit = profit[j+2] if j+2 < LEN else 0
                cur_profit = prices[j] - prices[i] + cur_profit
                max_profit = max(max_profit, cur_profit)
            profit[i] = max(max_profit, profit[i+1] if i+1 < LEN else 0)
        return profit[0]
    
s = Solution()
print(s.maxProfit([2,1,4]))
print(s.maxProfit([1,2,3,0,2]))
        