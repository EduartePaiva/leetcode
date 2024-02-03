class Solution:
    def minCostClimbingStairs(self, cost: list[int]) -> int:
        first,second = 0,0
        for i in range(len(cost)-1,-1,-1): 
            first,second = cost[i] + min(first,second),first
        return min(first,second)


s = Solution()
print(s.minCostClimbingStairs([1,100,1,1,1,100,1,1,100,1]))