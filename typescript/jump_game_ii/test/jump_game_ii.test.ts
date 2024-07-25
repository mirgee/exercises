import { jump_game_ii } from '../src/index_brute'

test('t0', () => {
  const expected = 2
  const actual = jump_game_ii([2,3,1,1,4])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 2
  const actual = jump_game_ii([2,3,0,1,4])
  expect(actual).toEqual(expected)
})
