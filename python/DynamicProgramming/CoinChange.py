class Solution:
    def coinChange(self, coins: list[int], amount: int) -> int:
        if amount == 0: return 0
        mem = {}
        coins.sort()
        def dfs(qtd:int) -> int:
            if qtd in mem: return mem[qtd]
            lowest = float('inf')
            for coin in coins:
                falta = qtd-coin
                if falta > 0:
                    minimo = dfs(falta)
                    if minimo != -1:
                        lowest = min(lowest,minimo)
                elif falta == 0:
                    mem[qtd] = 1
                    return 1
                else: break
            if lowest == float('inf'):
                mem[qtd] = -1
                return -1
            else:
                mem[qtd] = 1+lowest
                return 1+lowest
        return dfs(amount)
    

s = Solution()
print(s.coinChange([474,83,404,3],264))