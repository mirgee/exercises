import { one_bit_and_two_bit_characters } from '../src'

test('one_bit_and_two_bit_characters test 0', () => {
  const expected = null;
  const actual = one_bit_and_two_bit_characters()
  expect(actual).toEqual(expected)
})
