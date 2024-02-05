from collections import defaultdict
class Solution:
    def findItinerary(self, tickets: list[list[str]]) -> list[str]:
        tickets.sort()
        myDict = defaultdict(list)
        res = []
        for f, t in tickets:
            myDict[f].append(t)

        res.append('JFK')
        def dfs(city:str):
            if len(res) == len(tickets)+1: return True
            
            lista = myDict[city]
            for i in range(len(lista)):
                if lista[i] == '': continue
                cit = lista[i]
                lista[i] = ''
                res.append(cit)
                if dfs(cit): return True
                res.pop()
                lista[i] = cit
            return False
        dfs('JFK')
        return res