import { remove_element } from '../src'

test('t0', () => {
  const expected = 2
  const nums = [3,2,2,3]
  const actual = remove_element(nums, 3)
  expect(actual).toEqual(expected)
  expect(nums.slice(0, actual)).toEqual([2,2])
})

test('t1', () => {
  const expected = 5
  const nums = [0,1,2,2,3,0,4,2]
  const actual = remove_element(nums, 2)
  expect(actual).toEqual(expected)
  expect(new Set(nums.slice(0, actual))).toEqual(new Set([0,0,1,3,4]))
})
