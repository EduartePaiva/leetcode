class Solution:
    def solve(self, board: list[list[str]]) -> None:
        ROWS,COLS = len(board), len(board[0])

        def visitBorder(r:int, c:int):
            if r < 0 or c < 0 or r == ROWS or c == COLS or board[r][c] != 'O':
                return
            board[r][c] = 'T'
            visitBorder(r+1,c)
            visitBorder(r-1,c)
            visitBorder(r,c+1)
            visitBorder(r,c-1)

        for r in range(ROWS):
            visitBorder(r,0)
            visitBorder(r,COLS-1)
        for c in range(COLS):
            visitBorder(0,c)
            visitBorder(ROWS-1,c)
            
        for r in range(ROWS):
            for c in range(COLS):
                if board[r][c] == 'O':
                    board[r][c] = 'X'
                elif board[r][c] == 'T':
                    board[r][c] = 'O'

s = Solution()

bord = [["O","O","O","O","X","X"],["O","O","O","O","O","O"],["O","X","O","X","O","O"],["O","X","O","O","X","O"],["O","X","O","X","O","O"],["O","X","O","O","O","O"]]
bord = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
for row in bord:
    print(row)
s.solve(bord)
print('------------------')
for row in bord:
    print(row)
