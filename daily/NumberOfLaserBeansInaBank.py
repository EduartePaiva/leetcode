class Solution:
    def numberOfBeams(self, bank: list[str]) -> int:
        res = 0

        startRow = 0
        prevCells,curCells = 0,0
        for row in bank:
            prevCells = row.count('1')
            startRow+=1
            if prevCells > 0: break

        for i in range(startRow,len(bank)):
            curCells = bank[i].count('1')
            if curCells > 0:
                res += prevCells*curCells
                prevCells = curCells
                curCells = 0
        return res
     
            