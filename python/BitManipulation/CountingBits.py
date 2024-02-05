class Solution:
    def countBits(self, n: int) -> list[int]:
        res = [0] * (n+1)
        offset = 1
        offTimes2 = offset << 1
        for i in range(1,n+1):
            if offTimes2 == i:
                offset = i
                offTimes2 = i << 1
            res[i] = 1 + res[i-offset]
        return res



s = Solution()
print(s.countBits(8))