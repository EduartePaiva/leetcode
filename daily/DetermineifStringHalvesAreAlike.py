class Solution:
    def halvesAreAlike(self, s: str) -> bool:
        half = len(s) // 2
        vowels = set(['a','e','i','o','u','A','E','I','O','U'])
        num1,num2 = 0,0
        for i in range(half):
            if s[i] in vowels:
                num1+=1
        for i in range(half,len(s)):
            if s[i] in vowels:
                num2+=1
        return num1 == num2