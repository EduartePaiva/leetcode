class Solution:
    def jobScheduling(self, startTime: list[int], endTime: list[int], profit: list[int]) -> int:
        #naive recursive
        fusedJob = list(sorted(zip(startTime,endTime,profit),key=lambda x: x[0]))
        mem = {}
        # I can eather include current job and dfs the next non overlapping job
        # or I can dfs the next job
        def findNext(i) -> int:
            #binary search in findNext
            # find the next item wich the start is >= i end
            end = fusedJob[i][1]
            low,high = i+1,len(fusedJob)-1
            while low <= high:
                mid = low + (high-low) // 2
                if fusedJob[mid][0] >= end:
                    if fusedJob[mid-1][0] < end:
                        return mid
                    else:
                        high = mid - 1
                else:
                    low = mid+1
            return len(fusedJob)

            # skipping current and adding the next

        def dfs(i):
            if i in mem:return mem[i]
            if i >= len(fusedJob): return 0
            res = max(
                fusedJob[i][2] + dfs(findNext(i)),
                dfs(i+1)
            )
            mem[i] = res
            return res
        return dfs(0)
        

s = Solution()
#print(s.jobScheduling([1,2,3,3],[3,4,5,6],[50,10,40,70]))
print(s.jobScheduling([1,2,3,4,6],[3,5,10,6,9],[20,20,100,70,60]))