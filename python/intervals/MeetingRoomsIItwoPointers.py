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
        start = sorted(i.start for i in intervals)
        end = sorted(i.end for i in intervals)
        count,res = 0,0
        s,e = 0,0
        while s < len(intervals):
            if start[s] < end[e]:
                s+=1
                count+=1
                res=max(res,count)
            else:
                e+=1
                count-=1
        return res
    



