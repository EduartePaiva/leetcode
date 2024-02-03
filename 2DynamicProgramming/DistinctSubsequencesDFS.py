class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        dp = {}

        def dfs(si,ti):
            if ti == len(t):
                return 1
            if si == len(s):
                return 0
            if (si,ti) in dp:
                return dp[(si,ti)]
            
            # I have two options
            # I can include the next same character from s == t[ti] or I can skip it
            if s[si] == t[ti]:
                dp[(si,ti)] = dfs(si+1,ti+1) + dfs(si+1,ti)
            else:
                dp[(si,ti)] = dfs(si+1,ti)
            return dp[(si,ti)]
        
        return dfs(0,0)