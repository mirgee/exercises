import { h_index } from '../src'

test('t0', () => {
  const expected = 3
  const actual = h_index([3, 0, 6, 1, 5])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 1
  const actual = h_index([1, 3, 1])
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 6
  const actual = h_index([1, 1, 3, 6, 7, 10, 7, 1, 8, 5, 9, 1, 4, 4, 3])
  expect(actual).toEqual(expected)
})

test('t3', () => {
  const expected = 3
  const actual = h_index([25, 8, 5, 3, 3])
  expect(actual).toEqual(expected)
})

test('t4', () => {
  const expected = 1
  const actual = h_index([1])
  expect(actual).toEqual(expected)
})
