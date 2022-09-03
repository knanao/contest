package main

type Node struct {
	Val  int
	Next *Node
}

func insert(aNode *Node, x int) *Node {
	if aNode == nil {
		dummy := &Node{Val: x}
		dummy.Next = dummy
		return dummy
	}

	prev := aNode
	curr := aNode.Next
	toInsert := false

	for {
		if prev.Val <= x && x <= curr.Val {
			toInsert = true
		} else if prev.Val > curr.Val {
			if x >= prev.Val || x <= curr.Val {
				toInsert = true
			}
		}

		if toInsert {
			prev.Next = &Node{Val: x, Next: curr}
			return aNode
		}

		prev = curr
		curr = curr.Next

		if prev == aNode {
			break
		}
	}

	prev.Next = &Node{Val: x, Next: curr}
	return aNode
}
