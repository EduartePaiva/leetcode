class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: list[str]) -> int:
        #decision tree
        #each word beeing a word
        if endWord not in wordList:
            return 0


        wordList.append(beginWord)
        
        self.minPath = float('inf')
        visit = set()
        def oneLetterDif(w1:str, w2:str):
            cont = 0
            for i in range(len(w1)):
                if w1[i] != w2[i]:
                    cont +=1
            if cont == 1:return True
            else:return False


        def dfs(word:str,cont:int):
            if self.minPath < cont:
                return
            if word == beginWord:
                self.minPath = min(self.minPath,cont)
            
            for w in wordList:
                if w not in visit and oneLetterDif(word,w):
                    #print(word)
                    visit.add(w)
                    dfs(w,cont+1)
                    visit.discard(w)

        dfs(endWord,1)

        if self.minPath != float('inf'): return self.minPath
        else: return 0
    

s = Solution()
print(s.ladderLength('qa','sq',["si","go","se","cm","so","ph","mt","db","mb","sb","kr","ln","tm","le","av","sm","ar","ci","ca","br","ti","ba","to","ra","fa","yo","ow","sn","ya","cr","po","fe","ho","ma","re","or","rn","au","ur","rh","sr","tc","lt","lo","as","fr","nb","yb","if","pb","ge","th","pm","rb","sh","co","ga","li","ha","hz","no","bi","di","hi","qa","pi","os","uh","wm","an","me","mo","na","la","st","er","sc","ne","mn","mi","am","ex","pt","io","be","fm","ta","tb","ni","mr","pa","he","lr","sq","ye"]))