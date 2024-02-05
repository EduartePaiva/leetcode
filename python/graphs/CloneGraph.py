
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


from typing import Optional
class Solution:
    def __init__(self) -> None:
        self.myDict = {}
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if not node:
            return
        if node in self.myDict:
            return self.myDict[node]
        
        newNode = Node(node.val)
        self.myDict[node] = newNode

        for nodes in node.neighbors:
            newNode.neighbors.append(self.cloneGraph(nodes))
        return newNode