import { candy } from '../src'

test('t0', () => {
  const expected = 5
  const actual = candy([1,0,2])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 4
  const actual = candy([1,2,2])
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 11
  const actual = candy([1,3,4,5,2])
  expect(actual).toEqual(expected)
})
