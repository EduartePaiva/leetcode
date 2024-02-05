import heapq
class Solution:
    def kClosest(self, points: list[list[int]], k: int) -> list[list[int]]:
        # hashmap e priority queue
        # após alimentar os dados eu pop k times da priority queue e adiciono no res
        #a formula vai se simplificar a x1**2 + x2**2

        # pode haver repetição da mesma distância wtf

        heap = []
        for x,y in points:
            num = -(x*x + y*y)
            if len(heap) < k:
                heap.append((num,x,y))
                if len(heap) == k:
                    heapq.heapify(heap)
            else:
                heapq.heappushpop(heap,(num,x,y))
                
        return [[x,y] for _,x,y in heap]