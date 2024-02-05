import heapq
class Solution:
    def minCostConnectPoints(self, points: list[list[int]]) -> int:
        N = len(points)
        adj = {i:[] for i in range(N)}
        for i in range(N):
            for j in range(i+1,N):
                dist = abs(points[i][0]-points[j][0]) + abs(points[i][1]-points[j][1])
                adj[i].append((dist,j))
                adj[j].append((dist,i))
            
        minH = [(0,0)]
        visit = set()
        res = 0
        while len(visit) < N:
            cost, i = heapq.heappop(minH)
            if i in visit: continue
            visit.add(i)
            res+=cost
            for item in adj[i]:
                if item[1] in visit: continue
                heapq.heappush(minH,item)
        return res

        

s = Solution()
print(s.minCostConnectPoints([[0,0],[2,2],[3,10],[5,2],[7,0]]))
print(s.minCostConnectPoints([[-1000000,-1000000],[1000000,1000000]]))