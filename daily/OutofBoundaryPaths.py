class Solution:
    def findPaths(self, m: int, n: int, maxMove: int, startRow: int, startColumn: int) -> int:
        MODULO = pow(10,9)+7
        cache = {}
        def dfs(r,c,moves):
            if r < 0 or c < 0 or r == m or c == n:
                return 1
            if moves == 0:
                return 0
            if (r,c,moves) in cache:
                return cache[(r,c,moves)]
            
            val = dfs(r+1,c,moves-1)
            val += dfs(r-1,c,moves-1)
            val += dfs(r,c+1,moves-1)
            val += dfs(r,c-1,moves-1)
            val %=MODULO

            cache[(r,c,moves)] = val
            return val

        return dfs(startRow,startColumn,maxMove)

        


s = Solution()
print(s.findPaths(2,2,2,0,0))
print(s.findPaths(1,3,3,0,1))