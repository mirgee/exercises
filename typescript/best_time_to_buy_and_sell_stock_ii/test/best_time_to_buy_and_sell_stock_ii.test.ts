import { best_time_to_buy_and_sell_stock_ii } from '../src'

test('t0', () => {
  const expected = 7
  const actual = best_time_to_buy_and_sell_stock_ii([7,1,5,3,6,4])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 4
  const actual = best_time_to_buy_and_sell_stock_ii([1,2,3,4,5])
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 0
  const actual = best_time_to_buy_and_sell_stock_ii([7,6,4,3,1])
  expect(actual).toEqual(expected)
})
