class Solution:
    def rotate(self, matrix: list[list[int]]) -> None: 
        l,r  = 0, len(matrix)-1
        while l < r:
            for i in range(r-l):
                top, bottom = l,r

                topLeft = matrix[top][l+i]
                # move bottom left into top left
                matrix[top][l+i] = matrix[bottom-i][l]
                # move bottom right into bottom left
                matrix[bottom-i][l] = matrix[bottom][r-i]
                # move top right into bottom right
                matrix[bottom][r-i] = matrix[top+i][r]
                # mobe top left into top right
                matrix[top+i][r] = topLeft
            l+=1
            r-=1



        """
        Do not return anything, modify matrix in-place instead.
        """
        

matrix = [[1,2,3],[4,5,6],[7,8,9]]
for m in matrix:
    print(m)

s = Solution()
s.rotate(matrix)
print("------------------------------")
for m in matrix:
    print(m)