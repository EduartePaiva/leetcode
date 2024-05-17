package ArraysAndHashing

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_leet0027(t *testing.T) {
	nums := []int{3, 2, 2, 3}
	assert.Equal(t, removeElement(nums, 3), 2)
	assert.Equal(t, nums[0:2], []int{2, 2})
}
func Test_leet0027_2(t *testing.T) {
	nums := []int{0, 1, 2, 2, 3, 0, 4, 2}
	assert.Equal(t, removeElement(nums, 2), 5)
	assert.Equal(t, nums[0:5], []int{0, 1, 3, 0, 4})
}
