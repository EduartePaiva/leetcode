class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> bool:
        mem = {}
        def dfs(i:int):
            if i == len(s): return True
            if i in mem: return mem[i]
            for word in wordDict:
                if s[i:i+len(word)] == word:
                    if dfs(i+len(word)):
                        return True
            mem[i] = False
            return False
        return dfs(0)


        


s = Solution()
print(s.wordBreak("leetcode",["leet","code"]))
        