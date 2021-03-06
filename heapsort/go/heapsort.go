package heapsort

func Sort(a []int, f MaxHeapifyFunc) {
	h := f(a)
	for i := len(a) - 1; i >= 0; i-- {
		a[i] = h.DeleteMax()
	}
}
