# Definition for a binary tree node.
from typing import Optional
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def rangeSumBST(self, root: Optional[TreeNode], low: int, high: int) -> int:
        self.res = 0

        def dfs(node: TreeNode):
            if not node: return

            cur = node.val
            if cur >= low and cur <= high:
                self.res += cur
                dfs(node.left)
                dfs(node.right)
            elif cur < low:
                dfs(node.right)
            elif cur > high:
                dfs(node.left)
        dfs(root)
        return self.res
        