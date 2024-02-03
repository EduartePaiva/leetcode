class Solution:
    def partitionLabels(self, s: str) -> list[int]:
        #get all letters range
        intervals = {}
        for i, c in enumerate(s):
            if c in intervals:
                intervals[c][1] = i
            else:
                intervals[c] = [i,i]
        intervals = list(intervals.values())
        # now I have to fuse the intervals
        fused = [intervals[0]]
        for i in range(1,len(intervals)):
            prevLast = fused[-1][1]
            if intervals[i][0] <= prevLast:
                if prevLast < intervals[i][1]: 
                    fused[-1][1] = intervals[i][1]
            else:
                fused.append(intervals[i])
        return [j-i+1 for i,j in fused]

s = Solution()
print(s.partitionLabels("ababcbacadefegdehijhklij"))