import { trapping_rain_water } from '../src'

test('t0', () => {
  const expected = 6
  const actual = trapping_rain_water([0,1,0,2,1,0,1,3,2,1,2,1])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 9
  const actual = trapping_rain_water([4,2,0,3,2,5])
  expect(actual).toEqual(expected)
})
