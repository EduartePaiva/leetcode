import random

class RandomizedSet:

    def __init__(self):
        self.dict = {}
        self.array = []
        

    def insert(self, val: int) -> bool:
        if val in self.dict:
            return False
        self.dict[val] = len(self.array)
        self.array.append(val)
        return True

    def remove(self, val: int) -> bool:
        if val not in self.dict:
            return False
        oldIndex = self.dict[val]
        del self.dict[val]
        if oldIndex != len(self.array)-1:
            self.array[oldIndex] = self.array[-1]
            self.dict[self.array[oldIndex]] = oldIndex
        self.array.pop()
        return True

    def getRandom(self) -> int:
        randomNum = random.randint(0,len(self.array)-1)
        return self.array[randomNum]
        
        
s = RandomizedSet()
print(s.insert(0), s.insert(1), s.remove(0), s.insert(2), s.remove(1), s.getRandom())
