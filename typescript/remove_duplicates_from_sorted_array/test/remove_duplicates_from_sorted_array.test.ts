import { remove_duplicates_from_sorted_array } from '../src'

test('t0', () => {
  const expected = 2
  const nums = [1,1,2]
  const actual = remove_duplicates_from_sorted_array(nums)
  expect(actual).toEqual(expected)
  expect(nums.slice(0, actual)).toEqual([1,2])
})

test('t1', () => {
  const expected = 5
  const nums = [0,0,1,1,1,2,2,3,3,4]
  const actual = remove_duplicates_from_sorted_array(nums)
  expect(actual).toEqual(expected)
  expect(nums.slice(0, actual)).toEqual([0,1,2,3,4])
})
