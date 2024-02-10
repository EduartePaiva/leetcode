class Solution:
    def isHappy(self, n: int) -> bool:
        visit = set()
        while n != 1:
            sum,num = 0,n
            while num > 0:
                sum += (num%10)**2 
                num //= 10
            n = sum
            if n in visit:
                return False
            visit.add(n)
        return True

        


s = Solution()
print(s.isHappy(7))
print(s.isHappy(2))