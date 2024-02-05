# Definition for a binary tree node.
from typing import Optional
from collections import defaultdict
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        graph = defaultdict(list)
        def dfs(node:TreeNode):
            if not node: return
            if node.left:
                graph[node.val].append(node.left.val)
                graph[node.left.val].append(node.val)
            if node.right:
                graph[node.val].append(node.right.val)
                graph[node.right.val].append(node.val)
            dfs(node.left)
            dfs(node.right)
        dfs(root)

        def dfsCont(cur,prev,cont:int):
            lista = graph[cur]
            if len(lista) == 1 and lista[0] == prev:
                return cont
            maxVal = 0
            for val in lista:
                if val == prev:continue
                maxVal = max(maxVal,dfsCont(val,cur,cont+1))
            return maxVal
        
        return dfsCont(start,-1,0)

