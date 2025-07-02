package daily

func PossibleStringCount(word string) int {
	total := 0

	for i := 1; i < len(word); i++ {
		if word[i-1] == word[i] {
			total += 1
		}
	}
	return total + 1
}
