from collections import defaultdict

class Trie:
    def __init__(self):
        self.letters = defaultdict(Trie)
        self.isWord = False

    def insertWord(self,s:str):
        head = self
        for c in s:
            if c not in head.letters:
                head.letters[c] = Trie()
            head = head.letters[c]
        head.isWord = True

class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> bool:
        trie = Trie()
        mem = {}
        for word in wordDict:
            trie.insertWord(word)

        def dfs(i:int):
            if i == len(s): return True
            if i in mem: return mem[i]
            nt = trie
            while i < len(s) and s[i] in nt.letters:
                nt = nt.letters[s[i]]
                i+=1
                # tenho 2 opções, começar com uma nova word ou continuar
                if nt.isWord:
                    if dfs(i): 
                        mem[i] = True
                        return True
            mem[i] = False
            return False
        
        return dfs(0)


        


s = Solution()
print(s.wordBreak("leetcode",["leets","code"]))
        