# Definition for a binary tree node.
from typing import Optional
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        self.diff = 0
        
        def findMinMax(node:TreeNode):
            min1, max1 = findMinMax(node.left) if node.left else (node.val,node.val)
            min2, max2 = findMinMax(node.right) if node.right else (node.val,node.val)
            minimum = min(min1,min2)
            maximun = max(max1,max2)

            maxDiff = max(abs(node.val-minimum),abs(node.val-maximun))
            if maxDiff > self.diff:
                self.diff = maxDiff

            return (min(node.val,minimum),max(node.val,maximun))
        findMinMax(root)
        return self.diff
            
