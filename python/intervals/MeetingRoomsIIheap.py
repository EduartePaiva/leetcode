from typing import (
    List,
)
"""
Definition of Interval:
"""
class Interval(object):
    def __init__(self, start:int, end:int):
        self.start = start
        self.end = end

import heapq
class Solution:
    """
    @param intervals: an array of meeting time intervals
    @return: the minimum number of conference rooms required
    """
    def min_meeting_rooms(self, intervals: List[Interval]) -> int:
        if not intervals: return 0
        start,end = [],[]
        for inter in intervals:
            heapq.heappush(start,inter.start)
            heapq.heappush(end,inter.end)
        count,res = 0,0
        while start and end:
            if start[0] < end[0]:
                heapq.heappop(start)
                count+=1
                res=max(res,count)
            else:
                heapq.heappop(end)
                count-=1
        return res
    



