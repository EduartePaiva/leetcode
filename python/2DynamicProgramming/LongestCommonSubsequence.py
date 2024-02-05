class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        ROWS = len(text1)
        COLS = len(text2)
        dp = [[0 for _ in range(COLS+1)] for _ in range(ROWS+1)]

        for i in range(ROWS-1,-1,-1):
            for j in range(COLS-1,-1,-1):
                if text1[i] == text2[j]:
                    dp[i][j] = 1 + dp[i+1][j+1]
                else:
                    dp[i][j] = max(dp[i+1][j],dp[i][j+1])
        for r in dp:
            print(r)
        return dp[0][0]
s = Solution()
print(s.longestCommonSubsequence('abcde','ace'))

