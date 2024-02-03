class Solution:
    def longestIncreasingPath(self, matrix: list[list[int]]) -> int:
        ROWS,COLS = len(matrix),len(matrix[0])
        dp = [[0]* COLS for _ in range(ROWS)] 
        # (r,c) -> pathSize

        def path(r,c):
            if dp[r][c] != 0:
                return dp[r][c]
            
            maxPath = 0
            curVal = matrix[r][c]
            
            if r > 0 and matrix[r-1][c] > curVal:
                maxPath = max(maxPath,path(r-1,c))
            if c > 0 and matrix[r][c-1] > curVal:
                maxPath = max(maxPath,path(r,c-1))
            if r+1 < ROWS and matrix[r+1][c] > curVal:
                maxPath = max(maxPath,path(r+1,c))
            if c+1 < COLS and matrix[r][c+1] > curVal:
                maxPath = max(maxPath,path(r,c+1))
            maxPath+=1
            dp[r][c] = maxPath
            return maxPath
        
        res = 0
        for r in range(ROWS):
            for c in range(COLS):
                res = max(res,path(r,c))
        return res
            
            
            
            
        


s = Solution()
print(s.longestIncreasingPath([[9,9,4],[6,6,8],[2,1,1]]))