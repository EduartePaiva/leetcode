class Solution:
    def maxAreaOfIsland(self, grid: list[list[int]]) -> int:
        ROWS = len(grid)
        COLS = len(grid[0])
        maxArea = 0
        def countIsland(row:int,col:int):
            if row < 0 or row == ROWS or col < 0 or col == COLS or grid[row][col] == 0:
                return 0
            grid[row][col] = 0
            count = 1
            count+= countIsland(row+1,col)
            count+= countIsland(row-1,col)
            count+= countIsland(row,col+1)
            count+= countIsland(row,col-1)
            return count

        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c]:
                    maxArea = max(maxArea,countIsland(r,c))
        return maxArea