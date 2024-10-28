import { letter_combinations_of_a_phone_number } from '../src'

test('t0', () => {
  const expected = ["ad","ae","af","bd","be","bf","cd","ce","cf"];
  const actual = letter_combinations_of_a_phone_number("23");
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected: string[] = [];
  const actual = letter_combinations_of_a_phone_number("");
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = ["a","b","c"];
  const actual = letter_combinations_of_a_phone_number("2");
  expect(actual).toEqual(expected)
})
