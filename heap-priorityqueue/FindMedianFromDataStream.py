import heapq

class MedianFinder:

    def __init__(self):
        self.maxHeap = []
        self.minHeap = []
        

    def addNum(self, num: int) -> None:
        if self.minHeap and num < self.minHeap[0]:
            heapq.heappush(self.maxHeap,-num)
        else:
            heapq.heappush(self.minHeap,num)

        if abs(len(self.maxHeap) - len(self.minHeap)) > 1:
            if len(self.minHeap) < len(self.maxHeap):
                heapq.heappush(self.minHeap,-heapq.heappop(self.maxHeap))
            else:
                heapq.heappush(self.maxHeap,-heapq.heappop(self.minHeap))
        
    def findMedian(self) -> float:
        print(self.maxHeap, self.minHeap)
        if len(self.maxHeap) == len(self.minHeap):
            return (-self.maxHeap[0] + self.minHeap[0]) / 2
        elif len(self.maxHeap) > len(self.minHeap):
            return -self.maxHeap[0]
        else:
            return self.minHeap[0]
        

s = MedianFinder()
s.addNum(1)
s.addNum(2)
print(s.findMedian())
s.addNum(3)
print(s.findMedian())
# Your MedianFinder object will be instantiated and called as such:
# obj = MedianFinder()
# obj.addNum(num)
# param_2 = obj.findMedian()