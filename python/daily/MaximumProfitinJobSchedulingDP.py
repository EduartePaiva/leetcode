class Solution:
    def jobScheduling(self, startTime: list[int], endTime: list[int], profit: list[int]) -> int:
        fusedJob = list(sorted(zip(startTime,endTime,profit),key=lambda x: x[0]))
        def findNext(i) -> int:
            #binary search in findNext
            # find the next item wich the start is >= i end
            end = fusedJob[i][1]
            low,high = i+1,len(fusedJob)-1
            while low <= high:
                mid = low + (high-low) // 2
                if fusedJob[mid][0] >= end:
                    if fusedJob[mid-1][0] < end: return mid
                    else: high = mid - 1
                else: low = mid+1
            return -1

        dp = [None] * len(fusedJob)
        dp[-1] = fusedJob[-1][2]

        for i in range(len(fusedJob)-2,-1,-1):
            index = findNext(i)
            include = fusedJob[i][2]
            if index != -1:
                include+=dp[index]
            # adding current + the next non conflicting or add next
            dp[i] = max(include,dp[i+1])
        return dp[0]
        

s = Solution()
#print(s.jobScheduling([1,2,3,3],[3,4,5,6],[50,10,40,70]))
print(s.jobScheduling([1,2,3,4,6],[3,5,10,6,9],[20,20,100,70,60]))