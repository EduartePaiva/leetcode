import heapq
class Solution:
    def swimInWater(self, grid: list[list[int]]) -> int:
        BOUND = len(grid)
        visit = set([(0,0)])
        minH = [(grid[0][0],0,0)]
        passedTime = 0

        def helper(i,j):
            if (i,j) in visit or i >= BOUND or j >= BOUND or i < 0 or j < 0:
                return
            visit.add((i,j))
            heapq.heappush(minH,(grid[i][j],i,j))

        while minH:
            time, i, j = heapq.heappop(minH)
            passedTime = max(passedTime,time)
            if i == BOUND-1 and j == BOUND-1:
                break
            helper(i+1,j)
            helper(i-1,j)
            helper(i,j+1)
            helper(i,j-1)
        return passedTime


s = Solution()
#print(s.swimInWater([[0,2],[1,3]]))
print(s.swimInWater([[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]))