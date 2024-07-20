import { construct_k_palindrome_strings } from '../src'

// "anna" + "elble", "anbna" + "elle", "anellena" + "b"
test('t0', () => {
  const expected = true
  const actual = construct_k_palindrome_strings("annabelle", 2)
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = false
  const actual = construct_k_palindrome_strings("leetcode", 3)
  expect(actual).toEqual(expected)
})

// Each in separate string
test('t2', () => {
  const expected = true
  const actual = construct_k_palindrome_strings("true", 4)
  expect(actual).toEqual(expected)
})
