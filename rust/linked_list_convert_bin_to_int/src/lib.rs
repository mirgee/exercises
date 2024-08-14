// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

use std::convert::TryInto;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl From<Vec<i32>> for ListNode {
    fn from(v: Vec<i32>) -> Self {
        let mut head = ListNode::new(v[0]);
        let mut tail = &mut head;
        for i in v.into_iter().skip(0) {
            tail.next = Some(Box::new(ListNode::new(i)));
            tail = tail.next.as_mut().unwrap();
        }
        *head.next.unwrap()
    }
}

fn compute_len(head: &Option<Box<ListNode>>) -> i32 {
    let mut l = 0;
    if let Some(root) = head {
        l = 1;
        let mut node = root.clone();
        while let Some(next) = node.next {
            l += 1;
            node = next;
        }
    }
    return l;
    
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let l = compute_len(&head);
    let mut sum = 0;
    let mut i = 0;
    if let Some(root) = head {
        let mut node = root;
        i += 1;
        sum += node.val * 2_i32.pow((l-i).try_into().unwrap());
        while let Some(next) = node.next {
            i += 1;
            sum += next.val * 2_i32.pow((l-i).try_into().unwrap());
            node = next;
        }
    }
    sum
}

pub fn get_decimal_value_0(head: Option<Box<ListNode>>) -> i32 {
	let mut num = 0;
	let mut curr = &head;
	while let Some(node) = curr {
		num <<= 1;
		num |= node.val;
		curr = &node.next;
	}
	num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_convert_bin_to_int_0() {
        let expected = 5;
        let arg: ListNode = vec![1, 0, 1].into();
        let actual = get_decimal_value(Some(Box::new(arg)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_linked_list_convert_bin_to_int_1() {
        let expected = 0;
        let arg: ListNode = vec![0].into();
        let actual = get_decimal_value(Some(Box::new(arg)));
        assert_eq!(expected, actual);
    }
}
