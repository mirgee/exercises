import { {{ name }} } from '../src'

test('t0', () => {
  const expected = null;
  const actual = {{ name }}()
  expect(actual).toEqual(expected)
})
