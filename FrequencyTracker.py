class FrequencyTracker:
    # 2 hashmap, one for frequency of numbers and another for frequency of frequencies
    def __init__(self):
        self.num_frequency = {}
        self.freq_of_num_freq = {}
        

    def add(self, number: int) -> None:
        if number in self.num_frequency:
            freq = self.num_frequency[number]
            self.freq_of_num_freq[freq] -=1
            if self.freq_of_num_freq[freq] == 0:
                del self.freq_of_num_freq[freq]
            self.num_frequency[number] +=1
            freq = self.num_frequency[number]
            self.freq_of_num_freq[freq] = self.freq_of_num_freq.get(freq,0) + 1
        else:
            self.num_frequency[number] = 1
            self.freq_of_num_freq[1] = self.freq_of_num_freq.get(1,0) + 1
        

    def deleteOne(self, number: int) -> None:
        if number in self.num_frequency:
            freq = self.num_frequency[number]
            self.num_frequency[number] -=1
            if self.num_frequency[number] == 0:
                del self.num_frequency[number]
            self.freq_of_num_freq[freq] -=1
            if self.freq_of_num_freq[freq] == 0:
                del self.freq_of_num_freq[freq]
            if freq > 1:
                freq -=1
                self.freq_of_num_freq[freq] = self.freq_of_num_freq.get(freq,0) + 1
        

    def hasFrequency(self, frequency: int) -> bool:
        return frequency in self.freq_of_num_freq
        


# Your FrequencyTracker object will be instantiated and called as such:
# obj = FrequencyTracker()
# obj.add(number)
# obj.deleteOne(number)
# param_3 = obj.hasFrequency(frequency)