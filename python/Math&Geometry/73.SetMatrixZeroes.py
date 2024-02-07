class Solution:
    def setZeroes(self, matrix: list[list[int]]) -> None:
        ROWS,COLS = len(matrix),len(matrix[0])
        rows,cols = set(),set()
        for r in range(ROWS):
            for c in range(COLS):
                if matrix[r][c] == 0:
                    rows.add(r)
                    cols.add(c)

        for r in rows:
            for c in range(COLS):
                matrix[r][c] = 0
        for c in cols:
            for r in range(ROWS):
                matrix[r][c] = 0

        """
        Do not return anything, modify matrix in-place instead.
        """
        

m = [[1,1,1],[1,0,1],[1,1,1]]
s = Solution()
s.setZeroes(m)
for x in m:
    print(x)