from collections import defaultdict
class Solution:
    def findMatrix(self, nums: list[int]) -> list[list[int]]:
        myDict = defaultdict(int)
        res = []
        for num in nums:
                myDict[num] +=1

        for key in myDict:
            numNums = myDict[key]
            for i in range(numNums):
                if i >= len(res):
                    res.append([])
                res[i].append(key)
        return res


s = Solution()
print(s.findMatrix([1,3,4,1,2,3,1]))
            
