class Solution:
    def climbStairs(self, n: int) -> int:
        # decision tree?
        one,two = 1,1

        for _ in range(n-1):
            sm = one + two
            two = one
            one = sm
        return one
    
s = Solution()
print(s.climbStairs(5))