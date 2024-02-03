class Solution:
    def minCostClimbingStairs(self, cost: list[int]) -> int:
        self.res = float('inf')
        SIZE = len(cost)
        memo = {}

        # I can skip +2 or +1

        def dfs(index:int):
            if index >= SIZE: 
                return 0
            if index in memo:
                return memo[index]
            cur = min(dfs(index+1),dfs(index+2)) + cost[index]
            memo[index] = cur
            return cur

        return min(dfs(0),dfs(1))


s = Solution()
print(s.minCostClimbingStairs([1,100,1,1,1,100,1,1,100,1]))