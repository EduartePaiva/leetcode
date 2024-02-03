class Solution:
    def longestPalindrome(self, s: str) -> str:
        def insideOut(start:int,end:int):
            while start >=0 and end < len(s) and s[start] == s[end]:
                start-=1
                end+=1
            return s[start+1:end]
        
        longestString = ''
        for i in range(len(s)):
            tes1 = insideOut(i,i)
            tes2 = insideOut(i,i+1)
            if len(longestString) < len(tes1):
                longestString = tes1
            if len(longestString) < len(tes2):
                longestString = tes2
        return longestString
    
s = Solution()
print(s.longestPalindrome("babad"))
            