class Solution:
    def pacificAtlantic(self, heights: list[list[int]]) -> list[list[int]]:
        ROWS, COLS = len(heights), len(heights[0])
        res = []
        visitPacific = set()
        visitAtlantic = set()

        def visit(row:int,col:int, visitSet:set, prevH:int):
            if row < 0 or row == ROWS or col < 0 or col == COLS or (row,col) in visitSet or heights[row][col] < prevH:
                return
            visitSet.add((row,col))
            visit(row+1,col,visitSet,heights[row][col])
            visit(row-1,col,visitSet,heights[row][col])
            visit(row,col+1,visitSet,heights[row][col])
            visit(row,col-1,visitSet,heights[row][col])

        for c in range(COLS):
            visit(0,c,visitPacific,heights[0][c])
            visit(ROWS-1,c,visitAtlantic,heights[ROWS-1][c])
        for r in range(ROWS):
            visit(r,0,visitPacific,heights[r][0])
            visit(r,COLS-1,visitAtlantic,heights[r][COLS-1])

        for r in range(ROWS):
            for c in range(COLS):
                if (r,c) in visitAtlantic and (r,c) in visitPacific: 
                    res.append([r,c])
        return res
    
s = Solution()
print(s.pacificAtlantic([[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]))