class Solution:
    def canPartition(self, nums: list[int]) -> bool:
        half = sum(nums)
        if half%2 != 0: return False
        half = half//2
        mem = {}

        def dfs(qtd:int, index:int):
            if qtd == 0:
                return True
            elif qtd < 0:
                return False
            if index == len(nums): return False
            if (index,qtd) in mem: 
                return mem[(index,qtd)]

            # tenho 2 opções
            # eu posso incluir o atual ou pular para a proxima sem incluir
            res = dfs(qtd-nums[index],index+1) or dfs(qtd,index+1)
            mem[(index,qtd)] = res
            return res
        
        return dfs(half,0)

s = Solution()
print(s.canPartition([1,5,11,5]))