import { best_time_to_buy_and_sell_stock } from '../src'

test('t0', () => {
  const expected = 5
  const actual = best_time_to_buy_and_sell_stock([7,1,5,3,6,4])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 0
  const actual = best_time_to_buy_and_sell_stock([7,6,4,3,1])
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 1
  const actual = best_time_to_buy_and_sell_stock([1,2])
  expect(actual).toEqual(expected)
})
