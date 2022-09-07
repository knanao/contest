package main

type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func connect(root *Node) *Node {
	if root == nil {
		return nil
	}
	que := make([]*Node, 0, 0)
	que = append(que, root)
	for len(que) != 0 {
		var pre *Node
		for _, n := range que {
			que = que[1:]

			if pre != nil {
				pre.Next = n
			}
			pre = n
			if n.Left != nil {
				que = append(que, n.Left)
			}
			if n.Right != nil {
				que = append(que, n.Right)
			}
		}
	}
	return root
}
