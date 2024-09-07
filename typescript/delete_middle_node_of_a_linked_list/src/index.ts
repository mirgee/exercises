// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list

export class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function delete_middle_node_of_a_linked_list(head: ListNode | null): ListNode | null {
  if (head === null || head.next === null) return null

  let slow = head
  let fast = head.next


  while (fast.next !== null && fast.next.next !== null) {
    slow = slow.next!
    fast = fast.next.next
  }

  slow.next = slow.next === null ? null : slow.next.next

  return head
}
