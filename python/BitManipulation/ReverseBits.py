class Solution:
    def reverseBits(self, n: int) -> int:
        res = 0
        for _ in range(32):
            res = res << 1 | n & 1
            print(res)
            n = n >> 1
        return res

s = Solution()
print(s.reverseBits(8))