import { delete_middle_node_of_a_linked_list, ListNode } from '../src'

function arrayToLinkedList(arr: number[]): ListNode | null {
  if (arr.length === 0) {
    return null; // If the array is empty, return null
  }

  // Create the head of the linked list
  let head = new ListNode(arr[0]);
  let currentNode = head;

  // Iterate through the array and create the linked list
  for (let i = 1; i < arr.length; i++) {
    currentNode.next = new ListNode(arr[i]);
    currentNode = currentNode.next;
  }

  return head;
}

function linkedListToArray(head: ListNode | null): number[] {
  const nums = []
  let node = head
  while (node !== null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

test('t0', () => {
  const expected = [1,3,4,1,2,6]
  const actual = delete_middle_node_of_a_linked_list(arrayToLinkedList([1,3,4,7,1,2,6]))
  expect(linkedListToArray(actual)).toEqual(expected)
})

test('t1', () => {
  const expected = [1,2,4]
  const actual = delete_middle_node_of_a_linked_list(arrayToLinkedList([1,2,3,4]))
  expect(linkedListToArray(actual)).toEqual(expected)
})

test('t2', () => {
  const expected = [2]
  const actual = delete_middle_node_of_a_linked_list(arrayToLinkedList([2, 1]))
  expect(linkedListToArray(actual)).toEqual(expected)
})

test('t3', () => {
  const expected: number[] = []
  const actual = delete_middle_node_of_a_linked_list(arrayToLinkedList([1]))
  expect(linkedListToArray(actual)).toEqual(expected)
})
