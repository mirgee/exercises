import { length_of_last_word } from '../src'

test('t0', () => {
  const expected = 5
  const actual = length_of_last_word("Hello World")
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = 4
  const actual = length_of_last_word("   fly me   to   the moon  ")
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = 6
  const actual = length_of_last_word("luffy is still joyboy")
  expect(actual).toEqual(expected)
})
