class Solution:
    def countSubstrings(self, s: str) -> int:
        palindromics = set()
        def checkAndAdd(start,end):
            if s[start] == s[end]:
                start+=1
                end-=1
                if (end - start) > 0 and (start,end) in palindromics:
                    palindromics.add((start-1,end+1))
                    return True
                elif (end - start) <= 0:
                    palindromics.add((start-1,end+1))
                    return True
            return False
        
        for i in range(len(s)):
            # i é o window size
            # j começa no final do window size
            start = 0
            for end in range(i,len(s)):
                checkAndAdd(start,end)
                #print(s[start:end+1])
                start+=1
        #print(palindromics)
        return len(palindromics)




s = Solution()
print(s.countSubstrings('fdsklf'))