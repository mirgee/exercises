import { ListNode, maximum_twin_sum_of_a_linked_list } from '../src'

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
  const expected = 6
  const actual = maximum_twin_sum_of_a_linked_list(arrayToLinkedList([5, 4, 2, 1]))
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 7
  const actual = maximum_twin_sum_of_a_linked_list(arrayToLinkedList([4, 2, 2, 3]))
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 100001
  const actual = maximum_twin_sum_of_a_linked_list(arrayToLinkedList([1, 100000]))
  expect(actual).toEqual(expected)
})

test('t3', () => {
  const expected = 182
  const actual = maximum_twin_sum_of_a_linked_list(arrayToLinkedList([47, 22, 81, 46, 94, 95, 90, 22, 55, 91, 6, 83, 49, 65, 10, 32, 41, 26, 83, 99, 14, 85, 42, 99, 89, 69, 30, 92, 32, 74, 9, 81, 5, 9]))
  expect(actual).toEqual(expected)
})
