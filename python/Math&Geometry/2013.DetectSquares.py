from collections import defaultdict

class DetectSquares:

    def __init__(self):
        self.pointsCnt = defaultdict(int)
        self.xSet = defaultdict(set)
        
    def add(self, point: list[int]) -> None:
        self.pointsCnt[(point[0],point[1])] += 1
        self.xSet[point[0]].add(point[1])

    def count(self, point: list[int]) -> int:
        def countSquares(x, y1, y2):
            diff = abs(y1-y2)
            side1 = self.pointsCnt[(x,y2)]
            res = 0
            if (x+diff, y1) in self.pointsCnt and (x+diff, y2) in self.pointsCnt:
                res += (side1*self.pointsCnt[(x+diff, y1)]*self.pointsCnt[(x+diff, y2)])
            if (x-diff, y1) in self.pointsCnt and (x-diff, y2) in self.pointsCnt:
                res += (side1*self.pointsCnt[(x-diff, y1)]*self.pointsCnt[(x-diff, y2)])
            return res
        
        cont = 0
        x, y1 = point
        if x in self.xSet:
            for y2 in self.xSet[x]:
                if y2 == y1:
                    continue
                cont += countSquares(x,y1,y2)
        return cont
        
        


# Your DetectSquares object will be instantiated and called as such:
# obj = DetectSquares()
# obj.add(point)
# param_2 = obj.count(point)
            
s = DetectSquares()
s.add([3, 10])
s.add([11, 2])
s.add([3, 2])
print(s.count([11, 10])) # return 1. You can choose:
#   - The first, second, and third points
print(s.count([14, 8]))  # return 0. The query point cannot form a square with any points in the data structure.
s.add([11, 2])   # Adding duplicate points is allowed.
print(s.count([11, 10]))