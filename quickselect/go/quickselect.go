package quickselect

func Select(a []int, k int) int {
	p := partition(a)

	switch {
	case p > k:
		return Select(a[:p], k)

	case p < k:
		return Select(a[p+1:], k-p-1)

	default: // p == k
		return a[p]
	}
}

func partition(a []int) int {
	p := medianPivot(a)
	pv := a[p]

	last := len(a) - 1
	swap(a, p, last)

	firstHigh := 0
	for i, v := range a {
		if v < pv {
			swap(a, i, firstHigh)
			firstHigh++
		}
	}
	swap(a, last, firstHigh)

	return firstHigh
}

func swap(a []int, i, j int) {
	a[i], a[j] = a[j], a[i]
}

func medianPivot(a []int) int {
	mid := len(a) / 2
	end := len(a) - 1

	switch {
	case between(a[0], a[mid], a[end]):
		return 0
	case between(a[mid], a[0], a[end]):
		return mid
	default:
		return end
	}
}

func between(a, x, y int) bool {
	return x <= a && a <= y || y <= a && a <= x
}

/* randomPivot is an alternative to medianPivot, but medianPivot is recommended.
func randomPivot(a []int) int {
	return rand.Intn(len(a))
}
*/
