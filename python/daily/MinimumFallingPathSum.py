class Solution:
    def minFallingPathSum(self, matrix: list[list[int]]) -> int:
        ROWS = len(matrix)
        COLS = len(matrix[0])
        dp = [[0] * COLS for _ in range(ROWS)]

        def get_val(r,c):
            if c < 0 or c == COLS:
                return float('inf')
            return dp[r][c]
            
        for c in range(COLS):
            dp[ROWS-1][c] = matrix[ROWS-1][c]
        for r in range(ROWS-2,-1,-1):
            for c in range(COLS):
                dp[r][c] = matrix[r][c] + min(get_val(r+1,c),get_val(r+1,c+1),get_val(r+1,c-1))
        return min(dp[0])

s = Solution()
print(s.minFallingPathSum([[2,1,3],[6,5,4],[7,8,9]]))