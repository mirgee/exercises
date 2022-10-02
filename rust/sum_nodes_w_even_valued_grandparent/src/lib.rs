// Given the root of a binary tree, return the sum of values of nodes with an even-valued
// grandparent. If there are no nodes with an even-valued grandparent, return 0.
//
// A grandparent of a node is the parent of its parent if it exists.

use std::cell::RefCell;
use std::rc::Rc;

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

fn _sum_even_grandparent(node: Rc<RefCell<TreeNode>>, p: i32, gp: i32) -> i32 {
    let node = node.borrow();
    let val = node.val;
    let s1 = if let Some(left) = node.left.clone() {
        _sum_even_grandparent(left, val, p)
    } else { 0 };
    let s2 = if let Some(right) = node.right.clone() {
        _sum_even_grandparent(right, val, p)
    } else { 0 };
    if gp % 2 == 1 {
        val + s1 + s2
    } else {
        s1 + s2
    }
}

pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        _sum_even_grandparent(root, 0, 0)
    } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
