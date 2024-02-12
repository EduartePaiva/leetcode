class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        oneLeft = True
        for i in range(len(digits)-1,-1,-1):
            if digits[i] < 9:
                digits[i]+=1
                oneLeft = False
                break
            digits[i] = 0
        if oneLeft: 
            digits.insert(0,1)
        return digits
            
        

s = Solution()
print(s.plusOne([9,9,9,9]))