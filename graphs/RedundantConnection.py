class Solution:
    def findRedundantConnection(self, edges: list[list[int]]) -> list[int]:
        par = [i for i in range(len(edges) + 1)]
        rank = [1] * (len(edges) + 1)

        def find(n:int):
            p = par[n]

            while p != par[p]:
                par[p] = par[par[p]]
                p = par[p]
            return p
        
        # return false if can't complete
        def union(n1:int, n2:int):
            p1, p2 = find(n1), find(n2)
            if p1 == p2: return False

            if rank[p1] > rank[p2]:
                par[p2] = p1
                rank[p1] += rank[p2]
            else:
                par[p1] = p2
                rank[p2]+= rank[p1]
            return True

        for n1,n2 in edges:
            if not union(n1,n2): return [n1,n2]
    

s = Solution()

print(s.findRedundantConnection([[1,2],[2,3],[3,4],[1,4],[1,5]]))