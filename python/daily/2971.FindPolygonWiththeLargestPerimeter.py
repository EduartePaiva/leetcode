class Solution:
    def largestPerimeter(self, nums: list[int]) -> int:
        nums.sort()
        prefix_sum = []
        prefix_sum.append(nums[0])
        
        for i in range(1,len(nums)):
            prefix_sum.append(nums[i]+prefix_sum[-1])

        for i in range(len(nums)-1,1,-1):
            if prefix_sum[i-1] > nums[i]:
                return prefix_sum[i]
        return -1
    

s = Solution()
print(s.largestPerimeter([1,12,1,2,5,50,3]))
print(s.largestPerimeter([1,1,2]))

