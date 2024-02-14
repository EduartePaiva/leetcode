class Solution:
    def myPow(self, x: float, n: int) -> float:
        if n == 0: return 1
        if x == 0: return 0

        def helper(x:float,n:int) -> float:
            if n == 1: return x
            res = helper(x*x,n//2)
            return res*x if n%2 else res
        res = helper(x,abs(n))
        return 1/res if n < 0 else res
    
s = Solution()
print(s.myPow(2,10))

        