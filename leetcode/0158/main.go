package main

var solution = func(read4 func([]byte) int) func([]byte, int) int {
	pos, size := 0, 0
	b := make([]byte, 4)
	return func(buf []byte, n int) int {
		cnt := 0
		for cnt < n {
			if size == pos {
				pos = 0
				size = read4(b)
				if size == 0 {
					return cnt
				}
			}
			buf[cnt] = b[pos]
			cnt++
			pos++
		}
		return cnt
	}
}
