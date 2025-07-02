package daily

import (
	"github.com/stretchr/testify/assert"

	"testing"
)

func Test_leetcode3330(t *testing.T) {
	expect := 5
	result := PossibleStringCount("abbcccc")
	assert.Equal(t, expect, result)
}
