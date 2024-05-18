package ArraysAndHashing

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_leet0026(t *testing.T) {
	assert.Equal(t, removeDuplicates([]int{1, 1, 2}), 2)
}
