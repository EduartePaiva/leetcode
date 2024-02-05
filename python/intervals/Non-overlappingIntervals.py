class Solution:
    def eraseOverlapIntervals(self, intervals: list[list[int]]) -> int:
        intervals.sort(key=lambda x: x[1])
        second = intervals[0][1]
        res = 0
        for i in range(1,len(intervals)):
            if second > intervals[i][0]:
                second = min(second,intervals[i][1])
                res+=1
            else:
                second = intervals[i][1]
        return res