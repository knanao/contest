package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	var ret *TreeNode
	for root != nil {
		if p.Val >= root.Val {
			root = p.Right
		} else {
			ret = root
			root = p.Left
		}
	}
	return ret
}
