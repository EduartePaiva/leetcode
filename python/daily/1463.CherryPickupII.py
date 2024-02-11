class Solution:
    def cherryPickup(self, grid: list[list[int]]) -> int:
        # the brutest brute force
        ROWS,COLS = len(grid),len(grid[0])
        cache = {}

        def dfs(cur_row,bot1,bot2) -> int:
            if bot1==bot2 or bot1 < 0 or bot2 < 0 or bot1 == COLS or bot2 == COLS:
                return 0
            if cur_row == ROWS-1:
                return grid[cur_row][bot1] + grid[cur_row][bot2]
            if (cur_row,bot1,bot2) in cache:
                return cache[(cur_row,bot1,bot2)]
            
            res = grid[cur_row][bot1] + grid[cur_row][bot2]

            next_max=0
            for bt1 in range(-1,2):
                for bt2 in range(-1,2):
                    next_max = max(next_max,dfs(cur_row+1,bot1+bt1,bot2+bt2))
            res+=next_max
            cache[(cur_row,bot1,bot2)] = res
            return res
        return dfs(0,0,COLS-1)

            



s = Solution()
print(s.cherryPickup([[3,1,1],[2,5,1],[1,5,5],[2,1,1]]))
print(s.cherryPickup([[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]))