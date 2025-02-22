/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func recoverFromPreorder(traversal string) *TreeNode {
    input := parsePreorder(traversal)
	return dfsBuilder(&input, -1)
}

func parsePreorder(traversal string) []struct {
	val   int
	depth int
} {
	result := []struct {
		val   int
		depth int
	}{}

    depth, num := 0, ""
	inNumber := false

	for _, ch := range traversal {
		if ch == '-' {
			if inNumber {
				val, _ := strconv.Atoi(num)
				result = append(result, struct {
					val   int
					depth int
				}{val, depth})
				depth = 0
				num = ""
				inNumber = false
			}
			depth++
		} else {
			if !inNumber {
				inNumber = true
			}
			num += string(ch)
		}
	}

	if num != "" {
		val, _ := strconv.Atoi(num)
		result = append(result, struct {
			val   int
			depth int
		}{val, depth})
	}

	return result
}

func dfsBuilder(q *[]struct {
	val   int
	depth int
}, depth int) *TreeNode {
	if len(*q) > 0 && (*q)[0].depth == depth+1 {
		nodeData := (*q)[0]
		*q = (*q)[1:]
		node := &TreeNode{Val: nodeData.val}
		node.Left = dfsBuilder(q, depth+1)
		node.Right = dfsBuilder(q, depth+1)
		return node
	}
	return nil
}