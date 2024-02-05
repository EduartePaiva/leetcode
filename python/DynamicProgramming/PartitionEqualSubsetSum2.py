class Solution:
    def canPartition(self, nums: list[int]) -> bool:
        half = sum(nums)
        if half%2 != 0: return False
        half = half//2
        mem = set([nums.pop(),0])
        for i in range(len(nums)-1,-1,-1):
            li = []
            for val in mem:
                num = nums[i]+val
                if num < half: li.append(num)
                elif num == half: return True
            mem.update(li)
        print(mem)
        return False


        

s = Solution()
print(s.canPartition([2,5,12,5,2]))