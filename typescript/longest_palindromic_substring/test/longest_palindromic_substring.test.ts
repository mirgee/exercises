import { longest_palindromic_substring } from '../src'

test('t0', () => {
  const expected = "bab";
  const actual = longest_palindromic_substring("babad");
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = "bb";
  const actual = longest_palindromic_substring("cbbd");
  expect(actual).toEqual(expected)
})
