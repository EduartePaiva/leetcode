class Solution:
    def findOrder(self, numCourses: int, prerequisites: list[list[int]]) -> list[int]:
        # depth first search
        preMap = {i:[] for i in range(numCourses)}
        visit = set()
        res = []
        for key, requisite in prerequisites:
            preMap[key].append(requisite)
                
        def removeRequisits(key: int):
            if key not in preMap:
                return True
            if key in visit:
                return False
            visit.add(key)
            for req in preMap[key]:
                if not removeRequisits(req):
                    return False
            res.append(key)
            del preMap[key]
            return True
        for i in range(numCourses):
            if not removeRequisits(i):
                return []
        return res

        



s = Solution()
print(s.findOrder(2,[[1,0]]))