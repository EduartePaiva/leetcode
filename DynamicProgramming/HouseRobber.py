class Solution:
    def rob(self, nums: list[int]) -> int:
        # I can eather rob and skip +2
        # or I can skip +1
        #[2,7,9,3,1,0]
        nums.append(0)

        for i in range(len(nums)-3,-1,-1):
            nums[i] = max(nums[i]+nums[i+2],nums[i+1])
        print(nums)
        return nums[0]



s = Solution()
print(s.rob([2,7,9,3,1]))