package daily

import (
	"github.com/stretchr/testify/assert"

	"testing"
)

func Test_leetcode3300(t *testing.T) {
	expect := 48
	result := AreaOfMaxDiagonal([][]int{{9, 3}, {8, 6}})
	assert.Equal(t, expect, result)
}
