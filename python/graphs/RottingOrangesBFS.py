from collections import deque

class Solution:
    def orangesRotting(self, grid: list[list[int]]) -> int:
        ROWS,COLS = len(grid), len(grid[0])
        q = deque()
        self.fresh = 0

        def apodrece(r:int, c:int):
            if r < 0 or c < 0 or r == ROWS or c == COLS or grid[r][c] != 1:
                return False
            grid[r][c] = 2
            self.fresh-=1
            return True

        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c] == 2:
                    q.append((r,c))
                elif grid[r][c] == 1:
                    self.fresh+=1
        count = 0
        while q and self.fresh > 0:
            count+=1
            for _ in range(len(q)):
                r,c = q.popleft()
                if apodrece(r+1,c): q.append((r+1,c))
                if apodrece(r-1,c): q.append((r-1,c))
                if apodrece(r,c+1): q.append((r,c+1))
                if apodrece(r,c-1): q.append((r,c-1))

        return count if self.fresh == 0 else -1
    

s = Solution()
bord = [[2,1,1],[0,1,1],[1,0,1]]
for row in bord:
    print(row)
print(s.orangesRotting(bord))
print('------------------')
for row in bord:
    print(row)
