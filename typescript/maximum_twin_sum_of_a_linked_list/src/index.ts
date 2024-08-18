// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list

export class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function reverse_linked_list(head: ListNode | null): ListNode | null {
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

export function maximum_twin_sum_of_a_linked_list(head: ListNode | null): number {
  let middle = head
  let fast = head
  
  while (fast !== null) {
    fast = fast?.next?.next!
    middle = middle?.next!
  }

  let reversed = reverse_linked_list(middle)

  let max = 0
  let node = head
  let node_rev = reversed

  while (node_rev! !== null) {
    max = Math.max(max, node!.val + node_rev!.val)    
    node = node!.next
    node_rev = node_rev?.next!
  }

  return max
}
