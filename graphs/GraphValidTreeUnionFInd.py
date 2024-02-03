from typing import List

class Solution:
    def valid_tree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) < n-1:return False

        par = [i for i in range(n)]
        rank = [1] * n

        def find(n):
            while n != par[n]:
                par[n] = par[par[n]]
                n = par[n]
            return n

        def union(n1:int, n2:int):
            r1, r2 = find(n1), find(n2)
            if r1 == r2:return False

            if rank[r1] > rank[r2]:
                par[r2] = r1
                rank[r1] += rank[r2]
            else:
                par[r1] = r2
                rank[r2] += rank[r1]
            return True

        for n1, n2 in edges:
            if not union(n1, n2): return False
        return True



s = Solution()
print(s.valid_tree(5,[[0, 1], [2, 3], [1, 2], [3, 4]]))