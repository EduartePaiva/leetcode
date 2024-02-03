class Solution:
    def getSum(self, a: int, b: int) -> int:
        while b:
            tmp = a
            a = a ^ b
            b = (tmp & b) << 1
        return a





s = Solution()
print(s.getSum(9,11))