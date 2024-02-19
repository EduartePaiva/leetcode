class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1 == "0" or num2 == "0":
            return "0"
        if len(num1) < len(num2):
            num1,num2 = num2,num1
        res = [[0] * i for i in range(len(num2))]
        res.reverse()

        for i in range(len(num2)-1,-1,-1):
            val2 = int(num2[i])
            left = 0
            for j in range(len(num1)-1,-1,-1):
                n = int(num1[j])
                result = (val2*n)+left
                left = result // 10
                result %= 10
                res[i].append(result)
            if left > 0:
                res[i].append(left)
            res[i].reverse()

        #sum everything
        for v in res:
            print(v)
        left = 0
        allSum = []
        while res:
            total = left
            for i in range(len(res)):
                val = res[i].pop()
                total+=val
            left = total // 10
            allSum.append(total % 10)
            while res and not res[-1]:
                res.pop()
        if left > 0:
            allSum.append(left)
        allSum.reverse()
        return ''.join(str(n) for n in allSum)


        
        


s = Solution()
print(s.multiply('237','284'))