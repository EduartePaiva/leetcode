class Solution:
    def numDecodings(self, s: str) -> int:
        ways = set(str(i) for i in range(1,27))
        dp = {len(s): 1}

        def dfs(i) -> int:
            if i in dp: return dp[i]
            if i > len(s): return 0
            
            # there are two ways
            # skiping to the next +1
            # skipping to +2
            total = 0
            if s[i] in ways:
                total += dfs(i+1)
            if s[i:i+2] in ways:
                total += dfs(i+2)
            dp[i] = total
            return total
        res = dfs(0)
        print(dp)
        return res



s = Solution()
print(s.numDecodings('226'))