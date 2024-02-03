from typing import List
from collections import deque

class Solution:
    def walls_and_gates(self, rooms: List[List[int]]):
        ROWS,COLS, q = len(rooms), len(rooms[0]), deque()

        for r in range(ROWS):
            for c in range(COLS):
                if rooms[r][c] == 0:
                    q.append((r,c,0))
        
        def travel(r:int,c:int, dist:int) -> bool:
            if r < 0 or c < 0 or r == ROWS or c == COLS or rooms[r][c] != 2147483647:
                return
            rooms[r][c] = dist
            q.append((r,c,dist))

        while q:
            for _ in range(len(q)):
                r,c,dist = q.popleft()
                dist+=1
                travel(r+1,c,dist)
                travel(r-1,c,dist)
                travel(r,c+1,dist)
                travel(r,c-1,dist)
