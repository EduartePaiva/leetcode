from collections import defaultdict

class Solution:
    def minWindow(self, s: str, t: str) -> str:
        num_chars = defaultdict(int)
        for c in t:
            num_chars[c]+=1

        charSet = set(t)

        resS,resE = 0,len(s)
        start,end = 0,len(s)

        while start < len(s) and s[start] not in num_chars:
            start+=1
        for i in range(start,end):
            if s[i] in num_chars:
                num_chars[s[i]] -=1
                if num_chars[s[i]] == 0:
                    del num_chars[s[i]]
            if not num_chars:
                end = i
                break
        if num_chars:
            return ""
        resS,resE = start, end
        while True:
            charToLook = s[start]
            num_chars[charToLook]-=1
            if num_chars[charToLook] <= 0:
                del num_chars[charToLook]
            print(charToLook)
            start+=1
            while start < len(s) and s[start] not in charSet:
                start+=1
            if charToLook in num_chars:
                print(num_chars)
                num_chars[charToLook]-=1
                if num_chars[charToLook] == 0:
                    del num_chars[charToLook]
            else:
                while end < len(s) and s[end] != charToLook:
                    if s[end] in charSet:
                        num_chars[s[end]]+=1
                    end+=1
            print(start,end)
            if start == len(s) or end == len(s):
                break
            
            if (end-start) < (resE - resS):
                print("oi")
                resE,resS = end,start
        return s[resS:resE+1]



s = Solution()
print(s.minWindow("ADOBECODEBANC","ABC"))
# "ADOBECODEBANC"
#  0123456789