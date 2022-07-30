package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	c := make(map[*ListNode]struct{}, 0)
	for head != nil {
		if _, ok := c[head]; ok {
			return true
		}
		c[head] = struct{}{}
		head = head.Next
	}
	return false
}
