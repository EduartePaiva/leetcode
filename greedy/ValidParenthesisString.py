class Solution:
    def checkValidString(self, s: str) -> bool:
        leftMin = 0
        leftMax = 0

        for c in s:
            if c == '(':
                leftMin +=1
                leftMax +=1
                continue
            elif c == ')':
                leftMin -=1
                leftMax-=1
            else:
                leftMax+=1
                leftMin-=1
            if leftMax < 0: return False
            if leftMin < 0: leftMin+=1

        return leftMin == 0



s = Solution()
print(s.checkValidString("((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()"))