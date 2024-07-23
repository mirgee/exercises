import { majority_element } from '../src'

test('t0', () => {
  const expected = 3
  const actual = majority_element([3,2,3])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 2
  const actual = majority_element([2,2,1,1,1,2,2])
  expect(actual).toEqual(expected)
})
