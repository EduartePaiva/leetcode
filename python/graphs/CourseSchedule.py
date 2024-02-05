class Solution:
    def canFinish(self, numCourses: int, prerequisites: list[list[int]]) -> bool:
        # depth first search
        preMap = {}
        visit = set()
        for key, requisite in prerequisites:
            if key in preMap:
                preMap[key].append(requisite)
            else:
                preMap[key] = [requisite]
                
        def removeRequisits(key: int):
            if key not in preMap:
                return True
            if key in visit:
                return False
            visit.add(key)
            for req in preMap[key]:
                if not removeRequisits(req):
                    return False
            del preMap[key]
            return True
        for i in range(numCourses):
            if not removeRequisits(i):
                return False
        return True

        



s = Solution()
print(s.canFinish(2,[[1,0]]))