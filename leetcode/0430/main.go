package main

type Node struct {
	Val   int
	Prev  *Node
	Next  *Node
	Child *Node
}

func main() {}

func flatten(root *Node) *Node {
	if root == nil {
		return nil
	}

	dummy := &Node{Next: root}
	dfs(dummy, root)

	dummy.Next.Prev = nil
	return dummy.Next
}

func dfs(prev *Node, curr *Node) *Node {
	if curr == nil {
		return prev
	}
	curr.Prev = prev
	prev.Next = curr

	tmp := curr.Next

	tail := dfs(curr, curr.Child)
	curr.Child = nil

	return dfs(tail, tmp)
}
