// 217. Contains Duplicate
package ArraysAndHashing

func containsDuplicate(nums []int) bool {
	hashMap := make(map[int]bool)

	for _, num := range nums {
		if hashMap[num] {
			return true
		}
		hashMap[num] = true
	}
	return false
}
