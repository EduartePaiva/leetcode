from collections import Counter
from math import ceil
class Solution:
    def minOperations(self, nums: list[int]) -> int:
        freq,res = Counter(nums), 0
        for val in freq.values():
            if val == 1: return -1
            res+= ceil(val/3)
        return res


        
        

s = Solution()
print(s.minOperations([2,3,3,2,2,4,2,3,4]))