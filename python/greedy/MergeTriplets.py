class Solution:
    def mergeTriplets(self, triplets: list[list[int]], target: list[int]) -> bool:
        t0,t1,t2 = False,False,False
        for t in triplets:
            if t[0] > target[0] or t[1] > target[1] or t[2] > target[2]: continue
            if t[0] == target[0]: t0 = True
            if t[1] == target[1]: t1 = True
            if t[2] == target[2]: t2 = True
        return t0 and t1 and t2

s = Solution()
print(s.mergeTriplets([[2,5,3],[1,8,4],[1,7,5]],[2,7,5]))
