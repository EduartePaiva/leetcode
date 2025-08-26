package daily

import "math"

func AreaOfMaxDiagonal(dimensions [][]int) int {
	var diagonal float64
	area := 0

	for i, _ := range dimensions {
		l := dimensions[i][0]
		w := dimensions[i][1]

		newDiagonal := math.Sqrt(float64(l*l + w*w))
		newArea := l * w

		if (newDiagonal == diagonal && newArea < area) || newDiagonal < diagonal {
			continue
		}
		diagonal = newDiagonal
		area = newArea
	}

	return area

}
