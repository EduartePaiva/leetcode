# Definition for a binary tree node.
from typing import Optional
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        res1 = []
        res2 = []

        def dfs(node:TreeNode,arr:list):
            if not node:
                return
            if not node.left and not node.right:
                arr.append(node.val)
                return
            dfs(node.left,arr)
            dfs(node.right,arr)

        dfs(root1,res1)
        dfs(root2,res2)

        if len(res1) != len(res2):
            return False
        for i in range(len(res1)):
            if res1[i] != res2[i]:
                return False
        return True