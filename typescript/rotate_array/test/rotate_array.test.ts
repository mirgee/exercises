import { rotate_array } from '../src'

test('t0', () => {
  const expected = [5,6,7,1,2,3,4]
  const actual = rotate_array([1,2,3,4,5,6,7], 3)
  console.log(actual)
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = [3,99,-1,-100]
  const actual = rotate_array([-1,-100,3,99], 2)
  expect(actual).toEqual(expected)
})
