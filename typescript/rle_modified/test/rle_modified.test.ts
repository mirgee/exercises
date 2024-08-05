import { rle_modified } from '../src'

test('t0', () => {
  const expected = "aabcc3d4e"
  const actual = rle_modified("aabccdddeeee")
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = "abcd"
  const actual = rle_modified("abcd")
  expect(actual).toEqual(expected)
})

test('t2', () => {
  const expected = "a4bc"
  const actual = rle_modified("abbbbc")
  expect(actual).toEqual(expected)
})

test('t3', () => {
  const expected = ""
  const actual = rle_modified("")
  expect(actual).toEqual(expected)
})
