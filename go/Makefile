leetcode%:
	@id=$* ; \
	solution="daily/leetcode$${id}.go" ; \
	test="daily/leetcode$${id}_test.go" ; \
	if [ ! -f "$$solution" ]; then \
		echo "❌ Missing solution file: $$solution" ; \
		exit 1 ; \
	elif [ ! -f "$$test" ]; then \
		echo "❌ Missing test file: $$test" ; \
		exit 1 ; \
	else \
		echo "▶️  Running test: $$test with solution: $$solution" ; \
		go test -v $$test $$solution ; \
	fi

new:
	@id=$(filter-out $@,$(MAKECMDGOALS)) ; \
	solution="daily/leetcode$${id}.go" ; \
	test="daily/leetcode$${id}_test.go" ; \
	if [ -f "$$solution" ] || [ -f "$$test" ]; then \
		echo "❌ Files already exist for problem $$id" ; \
		exit 1 ; \
	fi ; \
	echo "📄 Creating: $$solution" ; \
	echo "package daily\n\n// leetcode $$id" > $$solution ; \
	echo "📄 Creating: $$test" ; \
	echo "package daily\n\
\n\
import (\n\
\t\"github.com/stretchr/testify/assert\"\n\n\
\t\"testing\"\n\
)\n\
\n\
func Test_leetcode$${id}(t *testing.T) {\n\
\texpect := 0\n\
\tresult := 0\n\
\tassert.Equal(t, expect, result)\n\
}" > $$test ; 
