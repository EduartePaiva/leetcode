import heapq
# posts vai guardar o postCount+tweetId

class User:
    def __init__(self):
        self.posts = []
        self.following = set()

class Twitter:
    def __init__(self):
        self.users = {}
        self.postsCount = 0

    def postTweet(self, userId: int, tweetId: int) -> None:
        if userId not in self.users:
            self.users[userId] = User()
        self.postsCount+=1
        self.users[userId].posts.append((self.postsCount,tweetId))

        
    def getNewsFeed(self, userId: int) -> list[int]:
        if userId not in self.users: return []

        user = self.users[userId]
        start, end = 0, len(user.posts)
        if len(user.posts) > 10: start = end-10
        topPosts = user.posts[start:end]
        heapq.heapify(topPosts)

        for followKey in user.following:
            followPosts = self.users[followKey].posts
            start, end = 0, len(followPosts)
            if len(followPosts) > 10: start = end-10
            for i in range(start,end):
                if len(topPosts) >=10:
                    heapq.heappushpop(topPosts,followPosts[i])
                else:
                    heapq.heappush(topPosts,followPosts[i])
        topPosts.sort(reverse=True,key=lambda x: x[0])
        return [x for  _,x in topPosts]

    def follow(self, followerId: int, followeeId: int) -> None:
        if not followeeId in self.users: self.users[followeeId] = User()
        if not followerId in self.users: self.users[followerId] = User()
        self.users[followerId].following.add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        self.users[followerId].following.discard(followeeId)
        


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)