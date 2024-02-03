import heapq
class Solution:
    def minInterval(self, intervals: list[list[int]], queries: list[int]) -> list[int]:
        intervals.sort(key=lambda x: x[0])
        minHeap = []
        res,i = {},0
        for q in sorted(queries):
            while i < len(intervals) and intervals[i][0] <= q:
                l,r = intervals[i]
                heapq.heappush(minHeap,(r-l+1,r))
                i+=1
            while minHeap and minHeap[0][1] < q:
                heapq.heappop(minHeap)
            res[q] = minHeap[0][0] if minHeap else -1
        return [res[i] for i in queries]
    
s = Solution()
print(s.minInterval([[2,3],[2,5],[1,8],[20,25]],[2,19,5,22]))
        
