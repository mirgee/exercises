import { one_bit_and_two_bit_characters } from '../src'

test('t0', () => {
  const expected = true;
  const actual = one_bit_and_two_bit_characters([1,0,0])
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = false;
  const actual = one_bit_and_two_bit_characters([1,1,1,0])
  expect(actual).toEqual(expected)
})
