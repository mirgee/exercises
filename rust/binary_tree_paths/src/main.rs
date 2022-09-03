use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

fn traverse(node: Rc<RefCell<TreeNode>>, path: &str, res: &mut Vec<String>) {
    let node = node.borrow();
    let path = format!("{}->{}", path, node.val);
    if node.left.is_none() && node.right.is_none() {
        res.push(path.clone());
    }
    if let Some(left) = node.left.clone() {
        traverse(left, &path, res);
    }
    if let Some(right) = node.right.clone() {
        traverse(right, &path, res);
    }
}

fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    if let Some(node) = root {
        let mut res = Vec::new();
        let node = node.borrow();
        let path = format!("{}", node.val);
        if node.left.is_none() && node.right.is_none() {
            res.push(path.clone());
        }
        if let Some(left) = node.left.clone() {
            traverse(left, &path, &mut res);
        }
        if let Some(right) = node.right.clone() {
            traverse(right, &path, &mut res);
        }
        res
    } else {
        Vec::new()
    }
}

fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}

fn example1() -> Option<Rc<RefCell<TreeNode>>> {
    let mut n1 = TreeNode::new(1);
    let mut n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n5 = TreeNode::new(5);
    n2.right = wrap(n5);
    n1.left = wrap(n2);
    n1.right = wrap(n3);
    return wrap(n1)
}

#[test]
fn test_binary_tree_paths1() {
    let root = example1();
    let res = binary_tree_paths(root);
    assert_eq!(res, vec!["1->2->5","1->3"]);
}

#[test]
fn test_binary_tree_paths2() {
    let root = wrap(TreeNode::new(1));
    let res = binary_tree_paths(root);
    assert_eq!(res, vec!["1"]);
}
