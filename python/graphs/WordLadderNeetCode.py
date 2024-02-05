from collections import defaultdict, deque

class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: list[str]) -> int:
        if endWord not in wordList:
            return 0
        
        nei = defaultdict(list)
        LENWORD = len(beginWord)
        wordList.append(beginWord)

        for word in wordList:
            for i in range(LENWORD):
                pattern = word[:i] + '*' + word[i+1:]
                nei[pattern].append(word)

        visit = set([endWord])
        q = deque([endWord])
        cnt = 1
        
        while q:
            print(q)
            for _ in range(len(q)):
                word = q.popleft()
                if word == beginWord:
                    return cnt
                for i in range(LENWORD):
                    pattern = word[:i] + '*' + word[i+1:]
                    for w in nei[pattern]:
                        if w not in visit:
                            q.append(w)
                            visit.add(w)
            cnt+=1
        return 0
    

s = Solution()
print(s.ladderLength('qa','.z',['.z',"si","go","se","cm","so","ph","mt","db","mb","sb","kr","ln","tm","le","av","sm","ar","ci","ca","br","ti","ba","to","ra","fa","yo","ow","sn","ya","cr","po","fe","ho","ma","re","or","rn","au","ur","rh","sr","tc","lt","lo","as","fr","nb","yb","if","pb","ge","th","pm","rb","sh","co","ga","li","ha","hz","no","bi","di","hi","qa","pi","os","uh","wm","an","me","mo","na","la","st","er","sc","ne","mn","mi","am","ex","pt","io","be","fm","ta","tb","ni","mr","pa","he","lr","sq","ye"]))