// 242. Valid Anagram
package ArraysAndHashing

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	sMap := make(map[rune]int)
	tMap := make(map[rune]int)

	for _, c := range s {
		sMap[c] += 1
	}
	for _, c := range t {
		tMap[c] += 1
	}
	for char, qtd := range sMap {
		if tMap[char] != qtd {
			return false
		}
	}

	return true
}
