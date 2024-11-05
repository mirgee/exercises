import { inorder_traversal, TreeNode } from '../src'

test('t0', () => {
  const expected = [];
  const actual = inorder_traversal(new TreeNode());
  expect(actual).toEqual(expected);
})

test('t1', () => {
  const expected = [1];
  const root = new TreeNode(1);
  const actual = inorder_traversal(root);
  expect(actual).toEqual(expected);
})
