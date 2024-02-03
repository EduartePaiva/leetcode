class Solution:
    def jump(self, nums: list[int]) -> int:
        END = len(nums)-1
        jumps = 0
        i = 0
        start = 0
        while i < END:
            if nums[i]+i >= END:
                return jumps + 1
            jumps+=1
            end = nums[i]+i
            toJump,maxSpace = i+1, i+1
            for j in range(end,start,-1):
                if j+nums[j] > maxSpace:
                    maxSpace = j+nums[j]
                    toJump = j
                if j+nums[j] >= END:
                    return jumps+1
            i = toJump
            start = end
        return END




s = Solution()
print(s.jump([2,3,1,1,4]))