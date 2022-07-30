package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	c := make(map[*ListNode]struct{}, 0)
	for head != nil {
		if _, ok := c[head]; ok {
			return head
		}
		c[head] = struct{}{}
		head = head.Next
	}
	return nil
}
