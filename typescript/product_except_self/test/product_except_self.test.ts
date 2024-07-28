import { product_except_self } from '../src'

test('t0', () => {
  const expected = [24,12,8,6]
  const actual = product_except_self([1,2,3,4])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = [0,0,9,0,0]
  const actual = product_except_self([-1,1,0,-3,3])
  expect(actual).toEqual(expected)
})
