class Solution:
    def coinChange(self, coins: list[int], amount: int) -> int:
        dp = [amount+1] * (amount+1)
        dp[0] = 0
        coins.sort()
        for a in range(1,amount+1):
            for c in coins:
                if a-c >= 0:
                    dp[a] = min(dp[a], 1+dp[a-c])
                else:
                    break
        print(dp)
        return dp[amount] if dp[amount] != amount+1 else -1
    

s = Solution()
print(s.coinChange([474,83,404,3],264))