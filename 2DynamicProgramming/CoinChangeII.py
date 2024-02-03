class Solution:
    def change(self, amount: int, coins: list[int]) -> int:
        coins.sort()
        i = 0
        while i < len(coins) and coins[i] <= amount:
            i+=1
        coins = coins[:i]

        dp = [0] * (amount+1)
        dp[0] = 1
        nextDP = [0] * (amount+1)
        nextDP[0] = 1

        for i in range(len(coins)-1,-1,-1):
            for a in range(1,amount+1):
                nextDP[a] = dp[a]
                if a - coins[i] >=0:
                    nextDP[a]+= nextDP[a-coins[i]]
                dp[a] = nextDP[a]
        return dp[amount]
                    


s = Solution()
print(s.change(5,[1,2,5]))

