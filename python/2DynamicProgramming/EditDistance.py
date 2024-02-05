class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        # if len(word1) > len(w2): I can replace and remove
        # if len(w1) < len(w2): I can replace and add
        # if len(w1) == len(w2): I can only replace
        dp = {}

        def dfs(i, j):
            if j == len(word2):
                return len(word1) - i
            if i == len(word1):
                return len(word2) - j
            if (i,j) in dp:
                return dp[(i,j)]
            
            replace,remove,add = 0,0,0
            replace = dfs(i+1,j+1)
            if word1[i] == word2[j]:
                dp[(i,j)] = replace
                return replace
            else:
                replace+=1
            remove = 1 + dfs(i+1,j)
            add = 1 + dfs(i,j+1)

            dp[(i,j)] = min(replace,remove,add)
            return dp[(i,j)]
        return dfs(0,0)
    
s = Solution()
print(s.minDistance("horse", "ros"))