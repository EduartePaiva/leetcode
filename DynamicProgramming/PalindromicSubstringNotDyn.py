class Solution:
    def countSubstrings(self, s: str) -> int:
        self.res = 0
        def checkAndAdd(start,end):
            while start > -1 and end < len(s) and s[start] == s[end]:
                self.res+=1
                start-=1
                end+=1
        for i in range(len(s)):
            checkAndAdd(i,i)
            checkAndAdd(i,i+1)
        return self.res




s = Solution()
print(s.countSubstrings('fdsklf'))