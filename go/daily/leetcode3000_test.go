package daily

import (
	"github.com/stretchr/testify/assert"

	"testing"
)

func Test_leetcode3300(t *testing.T) {
	expect := 48
	result := AreaOfMaxDiagonal([][]int{[]int{9, 3}, []int{8, 6}})
	assert.Equal(t, expect, result)
}
