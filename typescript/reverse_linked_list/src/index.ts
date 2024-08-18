// https://leetcode.com/problems/reverse-linked-list

export class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function reverse_linked_list(head: ListNode | null): ListNode | null {
  if (head === null) {
    return null
  }

  let node = head
  let node_next = head.next
  head.next = null

  while (node_next !== null) {
    let temp = node_next.next
    node_next.next = node
    node = node_next
    node_next = temp
  }

  return node
}
