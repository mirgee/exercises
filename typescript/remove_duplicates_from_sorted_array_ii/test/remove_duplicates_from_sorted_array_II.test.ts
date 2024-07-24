import { remove_duplicates_from_sorted_array_II } from '../src'

test('t0', () => {
  const expected = 5
  const nums = [1,1,1,2,2,3]
  const actual = remove_duplicates_from_sorted_array_II(nums)
  expect(actual).toEqual(expected)
  expect(nums.slice(0, actual)).toEqual([1,1,2,2,3])
})

test('t1', () => {
  const expected = 7
  const nums = [0,0,1,1,1,1,2,3,3]
  const actual = remove_duplicates_from_sorted_array_II(nums)
  expect(actual).toEqual(expected)
  expect(nums.slice(0, actual)).toEqual([0,0,1,1,2,3,3])
})
