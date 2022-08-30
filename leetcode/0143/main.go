package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	hash := make(map[*ListNode]struct{}, 0)
	for head != nil {
		if _, ok := hash[head]; ok {
			return head
		}
		hash[head] = struct{}{}
		head = head.Next
	}
	return nil
}
