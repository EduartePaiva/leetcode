from typing import List

class Solution:
    def valid_tree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) < n-1:
            return False
        
        # create list
        dict = {i:[] for i in range(n)}
        visit = set()
        for n1, n2 in edges:
            dict[n1].append(n2)
            dict[n2].append(n1)

        def dfs(n:int,prev:int):
            if n in visit: return False

            visit.add(n)

            for j in dict[n]:
                if j == prev: continue
                if not dfs(j,n): return False
            return True
        
        return dfs(0,-1) and len(visit) == n




s = Solution()
print(s.valid_tree(5,[[0, 1], [2, 3], [1, 2], [3, 4]]))