class Solution:
    def firstPalindrome(self, words: list[str]) -> str:
        def helper(s:str)->bool:
            half = len(s)//2
            r,l=0,0
            if len(s) % 2 != 0:
                r,l = half,half
            else:
                r,l = half-1,half
            while r > -1 and l < len(s):
                if s[r]!=s[l]:
                    return False
                r-=1
                l+=1
            return True
        for w in words:
            if helper(w): return w
        return ""
    
s = Solution()
print(s.firstPalindrome(["abc","car","ada","racecar","cool"]))