class Solution:
    def orangesRotting(self, grid: list[list[int]]) -> int:
        ROWS,COLS = len(grid), len(grid[0])
        #solução bture force, vai apodrecer, depois aplica a podridão
        def apodrece(r:int, c:int):
            if r < 0 or c < 0 or r == ROWS or c == COLS or grid[r][c] != 1:
                return False
            grid[r][c] = 3  
            return True
        apodreceu = True
        count = 0
        while apodreceu:
            apodreceu = False
            for r in range(ROWS):
                for c in range(COLS):
                    if grid[r][c] == 2:
                        a1 = apodrece(r+1,c)
                        a2 = apodrece(r-1,c)
                        a3 = apodrece(r,c+1)
                        a4 = apodrece(r,c-1)
                        apodreceu = apodreceu or a1 or a2 or a3 or a4
            if apodreceu:
                count+=1
                # aplica podridão
                for r in range(ROWS):
                    for c in range(COLS):
                        if grid[r][c] == 3: grid[r][c] = 2
        # checa se tem laranja left
        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c] == 1:
                    return -1
        return count
    

s = Solution()
bord = [[2,1,1],[1,1,0],[0,1,1]]
bord = [[0,2]]
bord = [[2,1,1],[1,1,1],[0,1,2]]
for row in bord:
    print(row)
print(s.orangesRotting(bord))
print('------------------')
for row in bord:
    print(row)
