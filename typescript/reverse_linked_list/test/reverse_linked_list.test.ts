import { ListNode, reverse_linked_list } from '../src'

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

test('t0', () => {
  const expected = arrayToLinkedList([5, 4, 3, 2, 1])
  const actual = reverse_linked_list(arrayToLinkedList([1, 2, 3, 4, 5]))
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = arrayToLinkedList([1, 2])
  const actual = reverse_linked_list(arrayToLinkedList([2, 1]))
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = arrayToLinkedList([])
  const actual = reverse_linked_list(arrayToLinkedList([]))
  expect(actual).toEqual(expected)
})
