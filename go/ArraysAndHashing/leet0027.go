// 27. Remove Element
package ArraysAndHashing

func removeElement(nums []int, val int) int {
	// return the number of values that are not equal to val
	// remove any val in nums
	toMove := 0
	for i := range len(nums) {
		if nums[i] != val {
			nums[toMove] = nums[i]
			toMove += 1
		}
	}
	return toMove
}
