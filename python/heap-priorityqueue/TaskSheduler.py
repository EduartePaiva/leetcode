import heapq
from collections import Counter, deque
class Solution:
    def leastInterval(self, tasks: list[str], n: int) -> int:
        count = Counter(tasks)
        heap = [-num for num in count.values()]
        deq = deque()
        heapq.heapify(heap)
        time = 0
        while heap or deq:
            time+=1
            if heap:
                val = heapq.heappop(heap) + 1
                if val < 0:
                    deq.append((val,time+n))
            if deq and deq[0][1] == time:
                heapq.heappush(heap,deq.popleft()[0])
        return time


    
        



s = Solution()
print(s.leastInterval(["A","A","A","B","B","B"],2))