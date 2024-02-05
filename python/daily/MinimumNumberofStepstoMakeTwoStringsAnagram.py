from collections import Counter

class Solution:
    def minSteps(self, s: str, t: str) -> int:
        first = Counter(s)
        second = Counter(t)
        res = 0

        for key in first:
            dif = first[key]-second[key]
            if dif > 0:
                res+=dif
        return res


        

s = Solution()
print(s.minSteps('leetcode','practice'))