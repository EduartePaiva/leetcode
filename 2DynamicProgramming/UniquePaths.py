class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        row = [1] * n
        for _ in range(m-1):
            for i in range(n-2,-1,-1):
                row[i]+=row[i+1]
        return row[0]

        


s = Solution()
print(s.uniquePaths(2,1))