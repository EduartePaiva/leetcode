import heapq

class Solution:
    def mostBooked(self, n: int, meetings: list[list[int]]) -> int:
        roomsCount = [0] * n
        unUsedRooms = [i for i in range(n)]
        meetingsInProgress = [] # (endTime,roomUsing)
        meetingsWaiting = [(i,j) for i,j in meetings]
        heapq.heapify(unUsedRooms)
        heapq.heapify(meetingsWaiting)

        cur_time = meetingsWaiting[0][0]
        while meetingsWaiting or meetingsInProgress:
            if meetingsInProgress:
                if cur_time >= meetingsInProgress[0][0]:
                    _, roomUsing = heapq.heappop(meetingsInProgress)
                    #print(roomUsing)
                    heapq.heappush(unUsedRooms, roomUsing)

            if unUsedRooms and meetingsWaiting:
                if meetingsWaiting[0][0] <= cur_time:
                    start, end = heapq.heappop(meetingsWaiting)
                    diff = end-start
                    cur_room = heapq.heappop(unUsedRooms)
                    heapq.heappush(meetingsInProgress,(cur_time+diff,cur_room))
                    roomsCount[cur_room]+=1
            #print(meetingsInProgress)
            #print(cur_time,meetingsInProgress)
            cur_time+=1
            if not unUsedRooms and meetingsWaiting:
                cur_time = max(cur_time,meetingsInProgress[0][0])
            

        return roomsCount.index(max(roomsCount))






s = Solution()
print(s.mostBooked(2,[[0,10],[1,5],[2,7],[3,4]]))
        