class Solution:
    def spiralOrder(self, matrix: list[list[int]]) -> list[int]:
        top = 0
        down = len(matrix)
        left = 0
        right = len(matrix[0])
        res = []
        while True:
            # 4 for after each for the space decreace

            # left to right for moving in columns
            for i in range(left,right):
                res.append(matrix[top][i])
            top += 1
            if top == down: break
            # moving from top to bottom at the right most
            for i in range(top,down):
                res.append(matrix[i][right-1])
            right -=1
            if right == left: break
            # moving from right to left at the bottomst position
            for i in range(right-1,left-1,-1):
                res.append(matrix[down-1][i])
            down-=1
            if top == down: break
            # moving from bottom to top from the left most
            for i in range(down-1,top-1,-1):
                res.append(matrix[i][left])
            left+=1
            if right == left: break
        return res




        


s = Solution()
print(s.spiralOrder([[1,2,3],[4,5,6],[7,8,9]]))