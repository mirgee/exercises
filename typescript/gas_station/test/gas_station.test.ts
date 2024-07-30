import { gas_station } from '../src'

test('t0', () => {
  const expected = 3
  const actual = gas_station([1,2,3,4,5], [3,4,5,1,2])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = -1
  const actual = gas_station([2,3,4], [3,4,3])
  expect(actual).toEqual(expected)
})
