class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        if (len(s1)+len(s2)) != len(s3):
            return False
        
        dp = {}
        def dfs(i1,i2):
            cur = i1+i2
            if cur == len(s3):
                return True
            if (i1,i2) in dp:
                return dp[(i1,i2)]
            
            if i1 < len(s1) and s3[cur] == s1[i1] and dfs(i1+1,i2):
                return True
            if i2 < len(s2) and s3[cur] == s2[i2] and dfs(i1,i2+1):
                return True
            dp[(i1,i2)] = False
            return False
        return dfs(0,0)
        

s = Solution()
print(s.isInterleave("aabcc","dbbca","aadbbcbcac"))
