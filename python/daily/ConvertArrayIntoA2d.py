class Solution:
    def findMatrix(self, nums: list[int]) -> list[list[int]]:
        res = []
        nums.sort()

        i=0
        while i<len(nums):
            num,j = nums[i], 0
            while i < len(nums) and num == nums[i]:
                if len(res) <= j:
                    res.append([])
                res[j].append(nums[i])
                i+=1
                j+=1
        return res

s = Solution()
print(s.findMatrix([1,3,4,1,2,3,1]))
            
