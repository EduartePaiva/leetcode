package ArraysAndHashing

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_leet0242(t *testing.T) {
	assert.Equal(t, isAnagram("anagram", "nagaram"), true)
	assert.Equal(t, isAnagram("rat", "car"), false)
}
