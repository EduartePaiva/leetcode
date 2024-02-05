from collections import Counter
import heapq
class Solution:
    def isNStraightHand(self, hand: list[int], groupSize: int) -> bool:
        if len(hand) % groupSize != 0: return False

        dict = Counter(hand)
        heap = list(dict.keys())
        heapq.heapify(heap)

        while heap:
            num = heap[0]
            for key in range(num,groupSize+num):
                if key not in dict: return False
                dict[key] -=1
                if dict[key] == 0:
                    popped = heapq.heappop(heap)
                    if popped != key: return False
        return True







s = Solution()
print(s.isNStraightHand([1,2,3,6,2,3,4,7,8,9],3))