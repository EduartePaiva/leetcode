class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        ROWS,COLS = len(word1), len(word2)
        dp = [[0] * (COLS+1) for _ in range(ROWS+1)]
        for i in range(ROWS-1,-1,-1):
            dp[i][COLS] = ROWS-i
        for i in range(COLS-1,-1,-1):
            dp[ROWS][i] = COLS-i


        for r in range(ROWS-1,-1,-1):
            for c in range(COLS-1,-1,-1):
                if word1[r] == word2[c]:
                    dp[r][c] = dp[r+1][c+1]
                else:
                    dp[r][c] = 1 + min(dp[r+1][c+1], dp[r][c+1], dp[r+1][c])
        return dp[0][0]
    
s = Solution()
print(s.minDistance("horse", "ros"))

#  r o s 0
# h
# o
# r
# s
# e
# 0