package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	hash := make(map[*ListNode]struct{}, 0)
	for headA != nil {
		hash[headA] = struct{}{}
		headA = headA.Next
	}

	for headB != nil {
		if _, ok := hash[headB]; ok {
			return headB
		}
		headB = headB.Next
	}
	return nil
}
