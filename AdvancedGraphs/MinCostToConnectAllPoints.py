from collections import defaultdict
import heapq
class Solution:
    def minCostConnectPoints(self, points: list[list[int]]) -> int:
        if len(points) == 2:
            return abs(points[0][0]-points[1][0]) + abs(points[0][1]-points[1][1])

        nodes = [(i,j) for i,j in points]
        adj = defaultdict(list)

        for i in range(len(nodes)):
            for j in range(i+1,len(nodes)):
                dist = abs(nodes[i][0]-nodes[j][0]) + abs(nodes[i][1]-nodes[j][1])
                adj[nodes[i]].append((dist,nodes[j]))
                adj[nodes[j]].append((dist,nodes[i]))
        minHeap = []
        cost = 0
        visit = set()
        cur = nodes[0]
        while len(visit) < len(nodes):
            visit.add(cur)
            lista = adj[cur]
            while lista:
                heapq.heappush(minHeap,lista.pop())
            dist = 0
            while cur in visit and minHeap:
                dist, cur = heapq.heappop(minHeap)
            if minHeap:
                cost+=dist
        return cost

        

s = Solution()
print(s.minCostConnectPoints([[-1000000,-1000000],[1000000,1000000]]))