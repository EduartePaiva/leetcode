from typing import (
    List,
)

class Solution:
    """
    @param n: the number of vertices
    @param edges: the edges of undirected graph
    @return: the number of connected components
    """
    def count_components(self, n: int, edges: List[List[int]]) -> int:
        # write your code here
        par = [i for i in range(n)]
        rank = [1] * n

        def findRoot(n):
            res = n
            while res != par[res]:
                par[res] = par[par[res]]
                res = par[res]
            return res
        def union(n1, n2):
            p1, p2 = findRoot(n1), findRoot(n2)
            if p1 == p2: return 0

            if rank[p1] > rank[p2]:
                par[p2] = p1
                rank[p1] +=rank[p2]
            else:
                par[p1] = p2
                rank[p2] += rank[p1]
            return 1

        cnt = n
        for n1, n2 in edges:
            cnt -= union(n1,n2)
        return cnt


s = Solution()
# print(s.count_components(6,[[0,1], [1,2], [2, 3], [4, 5]]))
# print(s.count_components(3,[[0,1], [0,2]]))
print(s.count_components(100,[[1, 67], [2, 34], [4, 69], [5, 24], [6, 78], [7, 58], [8, 62], [9, 64], [10, 5], [11, 45], [12, 81], [13, 27], [14, 61], [15, 91], [16, 95], [17, 42], [18, 27], [19, 36], [20, 91], [21, 4], [22, 2], [23, 53], [24, 92], [25, 82], [26, 21], [27, 16], [28, 18], [29, 95], [30, 47], [31, 26], [32, 71], [33, 38], [34, 69], [35, 12], [36, 67], [37, 99], [38, 35], [39, 94], [40, 3], [41, 11], [42, 22], [43, 33], [44, 73], [45, 64], [46, 41], [47, 11], [48, 53], [49, 68], [50, 47], [51, 44], [52, 62], [53, 57], [54, 37], [55, 59], [56, 23], [57, 41], [58, 29], [59, 78], [60, 16], [61, 35], [62, 90], [63, 42], [64, 88], [65, 6], [66, 40], [67, 42], [68, 64], [69, 48], [70, 46], [71, 5], [72, 90], [73, 29], [74, 70], [75, 50], [76, 6], [77, 1], [78, 93], [79, 48], [80, 29], [81, 23], [82, 84], [83, 54], [84, 56], [85, 40], [86, 66], [87, 76], [88, 31], [89, 8], [90, 44], [91, 39], [92, 26], [93, 23], [94, 37], [95, 38], [96, 18], [97, 82], [98, 29], [99, 41]]))
