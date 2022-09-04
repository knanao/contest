package main

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

var hash = make(map[*Node]*Node)

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}

	if v, ok := hash[head]; ok {
		return v
	}

	node := &Node{Val: head.Val}
	hash[head] = node

	node.Next = copyRandomList(head.Next)
	node.Random = copyRandomList(head.Random)

	return node
}
