// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree

export class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

export function lowest_common_ancestor_of_a_binary_tree(root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
  if (root === null) return null

  if (root === p || root === q) return root

  const left = lowest_common_ancestor_of_a_binary_tree(root?.left, p, q)
  const right = lowest_common_ancestor_of_a_binary_tree(root?.right, p, q)

  if (left !== null && right !== null) return root

  return left || right
}
