// 88. Merge Sorted Array
package ArraysAndHashing

func merge(nums1 []int, m int, nums2 []int, n int) {
	n1 := 0
	n2 := 0
	n1b := make([]int, m)
	copy(n1b, nums1[0:m])

	for i := range len(nums1) {
		if n1 < m && n2 < n {
			if n1b[n1] <= nums2[n2] {
				nums1[i] = n1b[n1]
				n1 += 1
			} else {
				nums1[i] = nums2[n2]
				n2 += 1
			}
		} else {
			if n1 < m {
				nums1[i] = n1b[n1]
				n1 += 1
			}
			if n2 < n {
				nums1[i] = nums2[n2]
				n2 += 1
			}
		}
	}
}
