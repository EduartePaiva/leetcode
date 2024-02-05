class Solution:
    def rob(self, nums: list[int]) -> int:
        def helper(ary:list) -> int:
            first,second = 0,0
            for i in range(len(ary)-1,-1,-1):
                tmp = first
                first = max(ary[i] + second, first)
                second = tmp
            return first
        return max(nums[0],helper(nums[1:]),helper(nums[:-1]))
    

s = Solution()
print(s.rob([1,2,3,1]))