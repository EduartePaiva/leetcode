class Solution:
    def findWinners(self, matches: list[list[int]]) -> list[list[int]]:
        ans1 = []
        ans2 = []

        winners = {}
        loosers = {}

        for w, l in matches:
            winners[w] = winners.get(w,0)+1
            loosers[l] = loosers.get(l,0)+1
        
        for key in winners:
            if key not in loosers:
                ans1.append(key)
        for key in loosers:
            if loosers[key] == 1:
                ans2.append(key)
        ans2.sort()
        ans1.sort()
        return [ans1,ans2]