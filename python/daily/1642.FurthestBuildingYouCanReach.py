import heapq

class Solution:
    def furthestBuilding(self, heights: list[int], bricks: int, ladders: int) -> int:
        heap = [] #will 
        for i in range(len(heights)-1):
            need = heights[i+1] - heights[i]
            if need <= 0: continue
            heapq.heappush(heap,need)
            ladders-=1
            if ladders == -1 and heap and heap[0] <= bricks:
                bricks-= heapq.heappop(heap)
                ladders+=1
            if ladders == -1:
                return i
        return len(heights)-1
                 

        


s = Solution()
print(s.furthestBuilding([4,12,2,7,3,18,20,3,19],10,2))