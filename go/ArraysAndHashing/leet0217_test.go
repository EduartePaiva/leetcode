package ArraysAndHashing

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_leet0217(t *testing.T) {
	assert.Equal(t, containsDuplicate([]int{1, 2, 3, 1}), true)
}
