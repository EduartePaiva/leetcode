
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def postorder(self, root: 'Node') -> list[int]:
        res = []
        def dfs(node):
            if not node:
                return
            for n in node.children:
                dfs(n)
            res.append(node.val)
        dfs(root)
        return res
        