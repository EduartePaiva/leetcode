class Solution:
    def canCompleteCircuit(self, gas: list[int], cost: list[int]) -> int:
        tank,started = 0,0

        for i in range(len(gas)):
            tank+= gas[i] - cost[i]
            if tank < 0:
                started,tank = i+1,0
        if started == len(gas):
            return -1
        for i in range(started):
            tank += gas[i] - cost[i]
            if tank < 0:
                return -1
        return started


s = Solution()
print(s.canCompleteCircuit([1,2,3,4,5],[3,4,5,1,2]))