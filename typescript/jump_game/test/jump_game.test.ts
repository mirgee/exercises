import { jump_game } from '../src/index_greedy'

test('t0', () => {
  const expected = true
  const actual = jump_game([2,3,1,1,4])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = false
  const actual = jump_game([3,2,1,0,4])
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = false
  const actual = jump_game([0, 1])
  expect(actual).toEqual(expected)
})

test('t3', () => {
  const expected = true
  const actual = jump_game([1, 2])
  expect(actual).toEqual(expected)
})

test('t4', () => {
  const expected = true
  const actual = jump_game([1])
  expect(actual).toEqual(expected)
})

test('t5', () => {
  const expected = true
  const actual = jump_game([0])
  expect(actual).toEqual(expected)
})
