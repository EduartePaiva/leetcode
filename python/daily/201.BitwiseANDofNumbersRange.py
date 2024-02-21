class Solution:
    def rangeBitwiseAnd(self, left: int, right: int) -> int:
        cnt = 0
        while left != right:
            left >>= 1
            right >>= 1
            cnt+=1
        
        return left << cnt
    

s = Solution()
print(s.rangeBitwiseAnd(1073741824,2147483647))