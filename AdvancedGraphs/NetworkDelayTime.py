import heapq
class Solution:
    def networkDelayTime(self, times: list[list[int]], n: int, k: int) -> int:
        adj = {i:[] for i in range(1,n+1)}
        for u,v,w in times:
            adj[u].append((w,v))

        minH = [(0,k)]
        res = 0
        visit = set()
        while minH and len(visit) < n:
            path,node = heapq.heappop(minH)
            if node in visit: continue
            visit.add(node)
            res = path
            for w,v in adj[node]:
                heapq.heappush(minH,(w+path,v))
        return res if len(visit) == n else -1
    
s = Solution()
print(s.networkDelayTime([[2,1,1],[2,3,1],[3,4,1]],4,2))
print(s.networkDelayTime([[1,2,1],[2,3,2],[1,3,2]],3,1))

