import { lowest_common_ancestor_of_a_binary_tree, TreeNode } from '../src'

function arrayToTree(arr: (number | undefined | TreeNode)[]): TreeNode | null {
  if (arr.length === 0) {
    return null;
  }

  // Create the root of the tree
  let root: TreeNode
  if (arr[0] instanceof TreeNode) {
    root = arr[0]
  } else {
    root = new TreeNode(arr[0]);
  }

  const queue: (TreeNode | null)[] = [root];

  let i = 1;
  while (i < arr.length) {
    const currentNode = queue.shift();

    if (currentNode !== null) {
      if (arr[i] !== null) {
        const n = arr[i]
        if (n instanceof TreeNode) {
          currentNode!.left = n;
        } else {
          currentNode!.left = new TreeNode(n) || null;
        }
        queue.push(currentNode!.left);
      } else {
        currentNode!.left = null;
      }
      i++;

      if (i < arr.length && arr[i] !== null) {
        const n = arr[i]
        if (n instanceof TreeNode) {
          currentNode!.right = n;
        } else {
          currentNode!.right = new TreeNode(n) || null;
        }
        queue.push(currentNode!.right);
      } else {
        currentNode!.right = null;
      }
      i++;
    }
  }

  return root;
}

test('t0', () => {
  const p = new TreeNode(5);
  const q = new TreeNode(1);
  const r = new TreeNode(3);
  const root = arrayToTree([r, p, q, 6, 2, 0, 8, undefined, undefined, 7, 4]);
  const actual = lowest_common_ancestor_of_a_binary_tree(root, p, q);
  expect(actual).toEqual(r)
})

test('t1', () => {
  const p = new TreeNode(5);
  const q = new TreeNode(4);
  const r = p;
  const root = arrayToTree([3, p, 1, 6, 2, 0, 8, undefined, undefined, 7, q]);
  const actual = lowest_common_ancestor_of_a_binary_tree(root, p, q);
  expect(actual).toEqual(r)
})
