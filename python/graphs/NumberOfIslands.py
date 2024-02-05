class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        ROWS = len(grid)
        COLS = len(grid[0])
        num = 0
        def destroyIsland(row:int,col:int):
            if row < 0 or row == ROWS or col < 0 or col == COLS or grid[row][col] == '0':
                return
            grid[row][col] = '0'
            destroyIsland(row+1,col)
            destroyIsland(row-1,col)
            destroyIsland(row,col+1)
            destroyIsland(row,col-1)

        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c] == '1':
                    num+=1
                    destroyIsland(r,c)
        return num