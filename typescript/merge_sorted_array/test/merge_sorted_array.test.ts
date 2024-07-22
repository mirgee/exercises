import { merge_sorted_array } from '../src'

test('t0', () => {
  const expected = [1,2,2,3,5,6];
  const actual = merge_sorted_array([1,2,3,0,0,0], 3, [2,5,6], 3)
  console.log(actual)
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = [1];
  const actual = merge_sorted_array([1], 1, [], 0)
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = [1];
  const actual = merge_sorted_array([0], 0, [1], 1)
  expect(actual).toEqual(expected)
})
