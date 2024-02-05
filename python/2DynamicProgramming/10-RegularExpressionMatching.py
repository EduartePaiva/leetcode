class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        # I can either use the * or not use the star
        cache = {}

        def dfs(i,j):
            if i == len(s) and j == len(p):
                return True
            if j == len(p):
                return False
            if (i,j) in cache:
                return cache[(i,j)]
            
            match = i < len(s) and (s[i] == p[j] or p[j] == ".")
            if j+1 < len(p) and p[j+1] == "*":
                cache[(i,j)] = (match and dfs(i+1,j)) or dfs(i,j+2)
                return cache[(i,j)]
            
            return dfs(i+1,j+1) if match else False
        return dfs(0,0)
            
        



s = Solution()

print("true:",s.isMatch("ab",'.*'))
print("false:",s.isMatch("aaa",'aaaa'))
print("true:",s.isMatch("aaa",'ab*a*c*a'))
print("false:",s.isMatch("aa",'a'))
print("true:",s.isMatch("aa",'a*'))
print("true:",s.isMatch("aab",'c*a*b'))
print("false:",s.isMatch("ab",'.*c'))
print("true:",s.isMatch("aaa",'a*a'))