class Solution:
    def singleNumber(self, nums: list[int]) -> int:
        xor = 0

        for n in nums:
            xor = xor ^ n
            print(xor)
        return xor
        

s = Solution()
print(s.singleNumber([2,2,1,3,4,5,5,4,3]))