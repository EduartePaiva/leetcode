from typing import List

class Interval(object):
    def __init__(self, start:int, end:int):
        self.start = start
        self.end = end

class Solution:
    """
    @param intervals: an array of meeting time intervals
    @return: if a person could attend all meetings
    """
    def can_attend_meetings(self, intervals: List[Interval]) -> bool:
        if not intervals: return True
        intervals.sort(key=lambda x: x.start)
        prevEnd = intervals[0].end

        for i in range(1,len(intervals)):
            if prevEnd > intervals[i].start:
                return False
            else:
                prevEnd = intervals[i].end
        return True