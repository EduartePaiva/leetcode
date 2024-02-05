class Solution:
    def insert(self, intervals: list[list[int]], newInterval: list[int]) -> list[list[int]]:
        ni = []
        for i in range(len(intervals)):
            f, s = intervals[i]
            #overshoot
            if newInterval[1] < f:
                ni.append(newInterval)
                return ni + intervals[i:]
            #undershoot
            elif newInterval[0] > s: 
                ni.append(intervals[i])
            # merge
            else:
                newInterval[0] = min(newInterval[0],f)
                newInterval[1] = max(newInterval[1],s)
        ni.append(newInterval)
        return ni











s = Solution()
print(s.insert([[1,2],[3,5],[6,7],[8,10],[12,16]],[4,8]))