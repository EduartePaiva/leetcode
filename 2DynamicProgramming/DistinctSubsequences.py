class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        ROWS,COLS = len(t), len(s)
        # check basecase
        size = 0
        for c in s:
            if c == t[size]:
                size+=1
                if size == len(t):
                    break
        if size != len(t):
            return 0


        cur = [0] * (COLS+1)
        prev = [0] * (COLS+1)
        prev[-1] = 1

        for r in range(ROWS-1,-1,-1):
            for c in range(COLS-1,-1,-1):
                if t[r] == s[c]:
                    cur[c] = prev[c+1] + cur[c+1]
                else:
                    cur[c] = cur[c+1]
            cur,prev = prev,cur

        print(cur,prev)
        return cur[0]


            

            

        

s = Solution()
print(s.numDistinct("rabbbit","rabbit"))
print(s.numDistinct("babgbag","bag"))