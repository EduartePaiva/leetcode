//26. Remove Duplicates from Sorted Array

package ArraysAndHashing

func removeDuplicates(nums []int) int {
	left := 0
	right := 0
	totalNums := len(nums)

	for right < len(nums) {
		for right < len(nums)-1 && nums[right] == nums[right+1] {
			right++
			totalNums--
		}
		nums[left] = nums[right]
		left++
		right++
	}
	return totalNums
}
