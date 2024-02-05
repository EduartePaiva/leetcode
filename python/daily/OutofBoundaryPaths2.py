class Solution:
    def findPaths(self, m: int, n: int, maxMove: int, startRow: int, startColumn: int) -> int:
        ROWS,COLS = m,n
        MODULO = pow(10,9)+7
        curGrid = [[0] * COLS for _ in range(ROWS)]
        prevGrid = [[0] * COLS for _ in range(ROWS)]

        def helper(r,c):
            if r < 0 or c < 0 or r == ROWS or c == COLS:
                return 1
            return prevGrid[r][c]

        for _ in range(maxMove):
            for r in range(ROWS):
                for c in range(COLS):
                    val = helper(r+1,c)
                    val += helper(r-1,c)
                    val += helper(r,c+1)
                    val += helper(r,c-1)
                    val %=MODULO
                    curGrid[r][c] = val
            prevGrid, curGrid = curGrid,prevGrid
        return prevGrid[startRow][startColumn]



        

        


s = Solution()
print(s.findPaths(2,2,2,0,0))
print(s.findPaths(1,3,3,0,1))