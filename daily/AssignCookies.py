class Solution:
    def findContentChildren(self, g: list[int], s: list[int]) -> int:
        g.sort()
        s.sort()
        res = 0

        child=0
        for cookie in s:
            if cookie >= g[child]:
                res+=1
                child+=1
                if child == len(g):
                    return res
        return res        
        

s = Solution()
print(s.findContentChildren([1,2,3],[1,1]))