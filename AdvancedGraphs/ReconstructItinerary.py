class Solution:
    def findItinerary(self, tickets: list[list[str]]) -> list[str]:
        tickets.sort(reverse=True)
        adj = {}
        res = []
        for f, t in tickets:
            if f in adj:
                adj[f].append(t)
            else:
                adj[f] = [t]

        def dfs(node:str):
            if node not in adj:
                res.append(node)
                return
            
            while adj[node]:
                dfs(adj[node].pop())
            res.append(node)
        dfs("JFK")
        res.reverse()
        return res
            


